use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, errors::ErrorCode, states::Config};

#[derive(Accounts)]
pub struct UpdateFeeRate<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn update_fee_rate_handler(ctx: Context<UpdateFeeRate>, fee_rate: u64) -> Result<()> {
    require!(fee_rate <= 10000, ErrorCode::FeeRateError);

    let mut config = ctx.accounts.config.load_mut()?;
    config.fee_rate = fee_rate;
    Ok(())
}
