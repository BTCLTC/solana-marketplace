pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod validate;

use crate::instructions::*;
use crate::validate::*;

use anchor_lang::prelude::*;

declare_id!("NFTMTNtLozbwJzvLDcdp2qRSgm4tKHxo2eu4cD3nC9y");

#[program]
pub mod solana_marketplace {
    use super::*;

    pub fn setup(ctx: Context<Setup>, nonce: u8, trade_fee_rate: u64) -> Result<()> {
        setup_handler(ctx, nonce, trade_fee_rate)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, trade_fee_rate: u64) -> Result<()> {
        update_config_handler(ctx, trade_fee_rate)
    }

    pub fn toggle_freeze_program(ctx: Context<ProgramFreeze>) -> Result<()> {
        feeze_program_handler(ctx)
    }

    #[access_control(start_sell_available(&ctx.accounts))]
    pub fn start_sell(ctx: Context<StartSell>, price: u64) -> Result<()> {
        start_sell_handle(ctx, price)
    }

    #[access_control(update_sell_available(&ctx.accounts))]
    pub fn update_sell(ctx: Context<UpdateSell>, price: u64) -> Result<()> {
        update_sell_handler(ctx, price)
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

