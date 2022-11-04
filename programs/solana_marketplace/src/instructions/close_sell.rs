use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::sysvar::rent;

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED},
    states::{Config, Sell},
};

#[event]
pub struct CloseSellEvent {
    order_id: u64,
    seller: Pubkey,
    nft_mint: Pubkey,
    nft_vault: Pubkey,
    created_at: u64,
}

#[derive(Accounts)]
pub struct CloseSell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

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
        constraint = user_nft_vault.mint == nft_mint.key(),
        constraint = user_nft_vault.owner == seller.key()
    )]
    pub user_nft_vault: Box<Account<'info, TokenAccount>>,

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

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn close_sell_handler(ctx: Context<CloseSell>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
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
                to: ctx.accounts.user_nft_vault.to_account_info(),
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
    // close event
    emit!(CloseSellEvent {
        order_id: config.order_id,
        seller: ctx.accounts.seller.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        nft_vault: ctx.accounts.nft_vault.key(),
        created_at: now_ts as u64,
    });

    Ok(())
}
