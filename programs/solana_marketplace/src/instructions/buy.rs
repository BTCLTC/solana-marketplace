use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::{sysvar::rent, program::invoke, system_instruction::transfer};

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED, TOKEN_VAULT_PDA_SEED},
    state::{Config, Sell}, utils::{assert_keys_equal, get_mint_from_token_account, get_owner_from_token_account},
};

#[derive(Accounts)]
#[instruction(_token_type: u8)]
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
        bump = config.load()?.nonce
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

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            TOKEN_VAULT_PDA_SEED.as_ref(),
            &[_token_type]
        ],
        bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer_token_wallet: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub seller_token_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = sell.load()?.owner == seller.key(),
        constraint = sell.load()?.nft_mint == nft_mint.key(),
        constraint = sell.load()?.nft_vault == nft_vault.key(),
        constraint = sell.load()?.token_type == _token_type,
        constraint = sell.load()?.owner_token_vault == seller_token_wallet.key(),
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

pub fn buy_handler(ctx: Context<Buy>, _token_type: u8) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    let mut sell = &mut ctx.accounts.sell.load_mut()?;

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

    let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
    if is_native {
        assert_keys_equal(
            ctx.accounts.buyer.key(),
            ctx.accounts.buyer_token_wallet.key(),
        )?;
        assert_keys_equal(
            ctx.accounts.seller.key(),
            ctx.accounts.seller_token_wallet.key(),
        )?;
        // send lamports to seller
        invoke(
            &transfer(
                ctx.accounts.buyer_token_wallet.to_account_info().key,
                ctx.accounts.seller_token_wallet.to_account_info().key,
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
                ctx.accounts.buyer_token_wallet.to_account_info().key,
                ctx.accounts.token_vault.to_account_info().key,
                fee,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    } else {
        // send fee token to token_vault
        let buyer_token_mint = get_mint_from_token_account(&ctx.accounts.buyer_token_wallet)?;
        let seller_token_mint = get_mint_from_token_account(&ctx.accounts.seller_token_wallet)?;
        let buyer_token_owner = get_owner_from_token_account(&ctx.accounts.buyer_token_wallet)?;
        let seller_token_owner = get_owner_from_token_account(&ctx.accounts.seller_token_wallet)?;

        assert_keys_equal(ctx.accounts.token_mint.key(), buyer_token_mint)?;
        assert_keys_equal(ctx.accounts.token_mint.key(), seller_token_mint)?;
        assert_keys_equal(ctx.accounts.buyer.key(), buyer_token_owner)?;
        assert_keys_equal(ctx.accounts.seller.key(), seller_token_owner)?;

        let cpi_fee_program = ctx.accounts.token_program.to_account_info();
        let cpi_price_program = ctx.accounts.token_program.to_account_info();
        let cpi_fee_accounts = Transfer {
            from: ctx.accounts.buyer_token_wallet.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_fee_ctx = CpiContext::new(cpi_fee_program, cpi_fee_accounts);
        token::transfer(cpi_fee_ctx, fee)?;

        let cpi_price_accounts = Transfer {
            from: ctx.accounts.buyer_token_wallet.to_account_info(),
            to: ctx.accounts.seller_token_wallet.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_price_ctx = CpiContext::new(cpi_price_program, cpi_price_accounts);
        token::transfer(cpi_price_ctx, price)?;
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

    //Update token config
    // TODO:
    // token_config.fee += fee;

    //Update config info
    config.count_sells -= 1;
    Ok(())
}
