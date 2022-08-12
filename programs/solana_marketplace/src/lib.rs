pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;
pub mod validate;

use crate::instructions::*;
use crate::validate::*;

use anchor_lang::prelude::*;

declare_id!("NFTMTNtLozbwJzvLDcdp2qRSgm4tKHxo2eu4cD3nC9y");

#[program]
pub mod solana_marketplace {
    use super::*;

    pub fn setup(ctx: Context<Setup>, bump: u8, fee_rate: u64) -> Result<()> {
        setup_handler(ctx, bump, fee_rate)
    }

    pub fn update_fee_account(ctx: Context<UpdateFeeAccount>) -> Result<()> {
        update_fee_account_handler(ctx)
    }

    pub fn update_fee_rate(ctx: Context<UpdateFeeRate>, fee_rate: u64) -> Result<()> {
        update_fee_rate_handler(ctx, fee_rate)
    }

    pub fn update_owner(ctx: Context<UpdateOwner>) -> Result<()> {
        update_owner_handler(ctx)
    }

    pub fn toggle_freeze_program(ctx: Context<ProgramFreeze>) -> Result<()> {
        toggle_feeze_program_handler(ctx)
    }

    #[access_control(start_sell_available(&ctx.accounts))]
    pub fn sell(ctx: Context<StartSell>, price: u64) -> Result<()> {
        sell_handle(ctx, price)
    }

    #[access_control(update_sell_available(&ctx.accounts))]
    pub fn update_sell_price(ctx: Context<UpdateSell>, price: u64) -> Result<()> {
        update_sell_price_handler(ctx, price)
    }

    #[access_control(close_sell_available(&ctx.accounts))]
    pub fn close_sell(ctx: Context<CloseSell>) -> Result<()> {
        close_sell_handler(ctx)
    }

    #[access_control(buy_available(&ctx.accounts))]
    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        buy_handler(ctx)
    }
}
