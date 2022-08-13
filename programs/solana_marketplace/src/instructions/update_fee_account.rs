use anchor_lang::prelude::*;

use crate::{states::Config, constants::CONFIG_PDA_SEED};

#[derive(Accounts)]
pub struct UpdateFeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account[constraint = config.load()?.fee_account.key() != fee_account.key()]]
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn update_fee_account_handler(ctx: Context<UpdateFeeAccount>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.fee_account = ctx.accounts.fee_account.key();
    Ok(())
}
