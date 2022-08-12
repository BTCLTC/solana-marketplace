use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, state::Config};

#[derive(Accounts)]
pub struct UpdateFeeAccount<'info> {
    #[account(address = config.load()?.owner)]
    pub owner: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn update_fee_account_handler(ctx: Context<UpdateFeeAccount>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.fee_account = ctx.accounts.fee_account.key();
    Ok(())
}
