use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, state::Config};

#[derive(Accounts)]
pub struct UpdateFeeRate<'info> {
    #[account(address = config.load()?.owner)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn update_fee_rate_handler(ctx: Context<UpdateFeeRate>, trade_fee_rate: u64) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.trade_fee_rate = trade_fee_rate;
    Ok(())
}
