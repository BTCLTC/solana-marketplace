use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, state::Config};

#[derive(Accounts)]
pub struct ProgramFreeze<'info> {
    #[account(address = config.load()?.owner)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn toggle_feeze_program_handler(ctx: Context<ProgramFreeze>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.freeze_program = !config.freeze_program;
    Ok(())
}
