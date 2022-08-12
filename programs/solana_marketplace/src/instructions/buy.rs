use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::{program::invoke, system_instruction::transfer, sysvar::rent};

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED},
    state::{Config, Sell},
};

#[derive(Accounts)]
pub struct Buy<'info> {
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
        bump = config.load()?.nonce,
        has_one=fee_account
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(
        constraint = nft_mint.supply == 1,
        constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

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
        mut,
        constraint = buyer_nft_vault.mint == nft_mint.key(),
        constraint = buyer_nft_vault.owner == buyer.key()
    )]
    pub buyer_nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = spl_token::native_mint::ID)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(address = config.load()?.fee_account)]
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = sell.load()?.owner == seller.key(),
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

    ///used by anchor for init of the token
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn buy_handler(ctx: Context<Buy>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    let sell = &mut ctx.accounts.sell.load()?;

    // Payment
    let fee: u64 = (sell.price as u128)
        .checked_mul(config.trade_fee_rate as u128)
        .unwrap()
        .checked_div(100)
        .unwrap()
        .try_into()
        .unwrap();

    let price: u64 = (sell.price as u128)
        .checked_sub(fee as u128)
        .unwrap()
        .try_into()
        .unwrap();

    // send lamports to seller
    invoke(
        &transfer(
            ctx.accounts.buyer.to_account_info().key,
            ctx.accounts.sell.to_account_info().key,
            price,
        ),
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // send lamports to fee_vault
    invoke(
        &transfer(
            ctx.accounts.buyer.to_account_info().key,
            ctx.accounts.fee_account.to_account_info().key,
            fee,
        ),
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.fee_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

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
    config.count_sells -= 1;
    Ok(())
}