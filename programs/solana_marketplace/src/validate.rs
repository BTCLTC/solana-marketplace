use crate::{errors::ErrorCode, Buy, CloseSell, StartSell, UpdateSell};
use anchor_lang::prelude::*;

pub fn start_sell_available(accounts: &StartSell) -> Result<()> {
    let config = accounts.config.load()?;

    if config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }

    Ok(())
}

pub fn update_sell_available(accounts: &UpdateSell) -> Result<()> {
    let config = accounts.config.load()?;
    let sell = accounts.sell.load()?;

    if config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}

pub fn close_sell_available(accounts: &CloseSell) -> Result<()> {
    let config = accounts.config.load()?;
    let sell = accounts.sell.load()?;

    if config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}

pub fn buy_available(accounts: &Buy) -> Result<()> {
    let config = accounts.config.load()?;
    let sell = accounts.sell.load()?;

    if config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}
