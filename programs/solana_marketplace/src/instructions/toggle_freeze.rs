use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, states::Config};

#[derive(Accounts)]
pub struct ProgramFreeze<'info> {
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

pub fn toggle_freeze_handler(ctx: Context<ProgramFreeze>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.freeze = !config.freeze;
    Ok(())
}
