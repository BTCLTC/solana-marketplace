use anchor_lang::prelude::*;

use crate::{constants::CONFIG_PDA_SEED, states::Config};

#[derive(Accounts)]
pub struct UpdateOwner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = owner.key() != new_owner.key())]
    pub new_owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,
}

pub fn update_owner_handler(ctx: Context<UpdateOwner>) -> Result<()> {
    let mut config = ctx.accounts.config.load_mut()?;
    config.owner = ctx.accounts.new_owner.key();
    Ok(())
}
