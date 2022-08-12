use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::sysvar::rent;

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED},
    errors::ErrorCode,
    states::{Config, Sell},
};

#[derive(Accounts)]
pub struct StartSell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(
        constraint = nft_mint.supply == 1,
        constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = user,
        token::mint = nft_mint,
        token::authority = nft_vault,
        seeds = [
            NFT_VAULT_PDA_SEED.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_nft_vault.mint == nft_mint.key(),
        constraint = user_nft_vault.owner == user.key()
    )]
    pub user_nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = spl_token::native_mint::ID)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = user,
        seeds = [
            SELL_PDA_SEED.as_ref(),
            user.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump,
        space = 8 + Sell::LEN
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

pub fn sell_handle(ctx: Context<StartSell>, price: u64) -> Result<()> {
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let mut config = ctx.accounts.config.load_mut()?;
    let mut sell = ctx.accounts.sell.load_init()?;

    require!(price > 0, ErrorCode::InvalidTokenAmount);

    // LOCK NFT : Transfer nft to nft_vault PDA
    {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_nft_vault.to_account_info(),
                to: ctx.accounts.nft_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, 1)?;
    }

    // Save Sell info
    sell.id = config.order_id;
    sell.owner = ctx.accounts.user.key();
    sell.nft_mint = ctx.accounts.nft_mint.key();
    sell.nft_vault = ctx.accounts.nft_vault.key();
    sell.price = price;
    sell.created_at = now_ts as u64;

    // Update config
    config.order_count += 1;
    config.order_id += 1;

    Ok(())
}
