use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount, PREFIX};
use solana_program::{program::invoke, system_instruction::transfer, sysvar::rent};

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED},
    errors::ErrorCode,
    states::{Config, Sell},
    validate::verify_metadata,
};

#[event]
pub struct BuyEvent {
    order_id: u64,
    buyer: Pubkey,
    seller: Pubkey,
    nft_mint: Pubkey,
    nft_vault: Pubkey,
    buyer_nft_vault: Pubkey,
    price: u64,
    created_at: u64,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        constraint = seller.key() != buyer.key(),
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub seller: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump,
        has_one=fee_account
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(
        constraint = nft_mint.supply == 1,
        constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [PREFIX.as_bytes(), mpl_token_metadata::ID.as_ref(), nft_mint.key().as_ref()],
        seeds::program = mpl_token_metadata::ID,
        bump
    )]
    pub nft_metadata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = nft_vault.mint == nft_mint.key(),
        seeds = [
            NFT_VAULT_PDA_SEED.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = sell.load()?.seller == seller.key(),
        constraint = sell.load()?.nft_mint == nft_mint.key(),
        constraint = sell.load()?.nft_vault == nft_vault.key(),
        seeds = [
            SELL_PDA_SEED.as_ref(),
            seller.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        close = seller,
        bump
    )]
    pub sell: AccountLoader<'info, Sell>,

    /// used by anchor for init of the token
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = spl_associated_token_account::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn buy_nft_handler<'info>(ctx: Context<'_, '_, '_, 'info, BuyNFT<'info>>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    let sell = &mut ctx.accounts.sell.load()?;

    let metadata: Metadata =
        Metadata::from_account_info(&ctx.accounts.nft_metadata.to_account_info())?;

    let mut total_seller_fee_basis_points: u128 = 0;

    if let Some(creators) = &metadata.data.creators {
        verify_metadata(creators)?;

        total_seller_fee_basis_points = (sell.price as u128)
            .checked_mul(metadata.data.seller_fee_basis_points as u128)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0);

        // Payment to shares
        for (index, item) in creators.iter().enumerate() {
            if item.share > 0 {
                let fee = total_seller_fee_basis_points
                    .checked_mul(item.share as u128)
                    .unwrap_or(0)
                    .checked_div(100)
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(0);

                let share_account_info_result = ctx.remaining_accounts.get(index);

                if let Some(share_account_info) = share_account_info_result {
                    require!(
                        share_account_info.key() == item.address,
                        ErrorCode::InvalidSharesPubkey
                    );

                    invoke(
                        &transfer(&ctx.accounts.buyer.key(), &item.address, fee),
                        &[
                            ctx.accounts.buyer.to_account_info(),
                            share_account_info.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                    )?;
                } else {
                    return err!(ErrorCode::PubkeyMiss);
                }
            }
        }
    }

    // Payment Platform service charge
    let mut platform_service_fee: u64 = 0;

    if config.fee_rate > 0 {
        platform_service_fee = (sell.price as u128)
            .checked_mul(config.fee_rate as u128)
            .unwrap_or(0)
            .checked_div(10000)
            .unwrap_or(0)
            .try_into()
            .unwrap_or(0);
    }

    let seller_receive_amount: u64 = (sell.price as u128)
        .checked_sub(platform_service_fee as u128)
        .unwrap_or(0)
        .checked_sub(total_seller_fee_basis_points)
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0);

    if seller_receive_amount > 0 {
        // send lamports to seller
        invoke(
            &transfer(
                &ctx.accounts.buyer.key(),
                &ctx.accounts.seller.key(),
                seller_receive_amount,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    if platform_service_fee > 0 {
        // send lamports to fee_vault
        invoke(
            &transfer(
                &ctx.accounts.buyer.key(),
                &ctx.accounts.fee_account.key(),
                platform_service_fee,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.fee_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    // Transfer nft to user from vault
    {
        let nft_vault_bump = *ctx.bumps.get("nft_vault").unwrap();
        let seeds = &[
            NFT_VAULT_PDA_SEED.as_ref(),
            ctx.accounts.nft_mint.to_account_info().key.as_ref(),
            &[nft_vault_bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.nft_vault.to_account_info(),
                to: ctx.accounts.buyer_nft_vault.to_account_info(),
                authority: ctx.accounts.nft_vault.to_account_info(),
            },
            signer,
        );
        token::transfer(cpi_ctx, 1)?;

        // Close nft vault
        let cpi_close_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: ctx.accounts.nft_vault.to_account_info(),
                destination: ctx.accounts.seller.to_account_info(),
                authority: ctx.accounts.nft_vault.to_account_info(),
            },
            signer,
        );
        token::close_account(cpi_close_ctx)?;
    }

    //Update config info
    config.order_count -= 1;

    let now_ts = Clock::get().unwrap().unix_timestamp;
    // buy event
    emit!(BuyEvent {
        order_id: config.order_id,
        buyer: ctx.accounts.buyer.key(),
        seller: ctx.accounts.seller.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        nft_vault: ctx.accounts.nft_vault.key(),
        buyer_nft_vault: ctx.accounts.buyer_nft_vault.key(),
        price: sell.price,
        created_at: now_ts as u64,
    });
    Ok(())
}
