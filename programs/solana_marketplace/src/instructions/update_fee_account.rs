use anchor_lang::{prelude::*, system_program};
use solana_program::{program::invoke, system_instruction::transfer};

use crate::{constants::CONFIG_PDA_SEED, states::Config};

#[derive(Accounts)]
pub struct UpdateFeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        constraint = config.load()?.fee_account.key() != fee_account.key()
    )]
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump,
        has_one=owner
    )]
    pub config: AccountLoader<'info, Config>,

    /// used by anchor for init of the token
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn update_fee_account_handler(ctx: Context<UpdateFeeAccount>) -> Result<()> {
    let fee_account = ctx.accounts.fee_account.to_account_info();
    if fee_account.lamports() == 0 {
        // send 0.001 to fee_account
        invoke(
            &transfer(
                &ctx.accounts.owner.key(),
                &ctx.accounts.fee_account.key(),
                1000000,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.fee_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    let mut config = ctx.accounts.config.load_mut()?;
    config.fee_account = ctx.accounts.fee_account.key();
    Ok(())
}
