use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token};
use solana_program::sysvar::rent;

use crate::{
    constants::{CONFIG_PDA_SEED, SELL_PDA_SEED},
    errors::ErrorCode,
    states::{Config, Sell},
};

#[derive(Accounts)]
pub struct UpdateSell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
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
        constraint = sell.load()?.seller == seller.key(),
        constraint = sell.load()?.nft_mint == nft_mint.key(),
        seeds = [
            SELL_PDA_SEED.as_ref(),
            seller.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
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

pub fn update_sell_price_handler(ctx: Context<UpdateSell>, price: u64) -> Result<()> {
    require!(price > 0, ErrorCode::InvalidTokenAmount);

    let now_ts = Clock::get().unwrap().unix_timestamp;
    let mut sell = ctx.accounts.sell.load_mut()?;
    sell.price = price;
    sell.updated_at = now_ts as u64;

    Ok(())
}
