use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Token};
use solana_program::{program::invoke, system_instruction::transfer, sysvar::rent};

use crate::{constants::CONFIG_PDA_SEED, errors::ErrorCode, states::Config};

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump,
        space = 16 + Config::LEN
    )]
    pub config: AccountLoader<'info, Config>,

    /// used by anchor for init of the token
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn setup_handler(ctx: Context<Setup>, bump: u8, fee_rate: u64) -> Result<()> {
    require!(fee_rate <= 10000, ErrorCode::FeeRateError);

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

    let mut config = ctx.accounts.config.load_init()?;
    config.owner = ctx.accounts.owner.key();
    config.fee_account = ctx.accounts.fee_account.key();
    config.order_count = 0;
    config.fee_rate = fee_rate;
    config.order_id = 1;
    config.freeze = false;
    config.bump = bump;
    Ok(())
}
