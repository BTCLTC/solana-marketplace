use anchor_lang::prelude::*;
use crate::{AcceptOffer, ApplyOffer, Buy, CancelOffer, CloseSell, ErrorCode, StartSell, UpdateSell};

pub fn start_sell_available(accounts: &StartSell) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    Ok(())
}

pub fn update_sell_available(accounts: &UpdateSell) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    if accounts.sell.offer_count > 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}

pub fn close_sell_available(accounts: &CloseSell) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}

pub fn buy_available(accounts: &Buy) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}

pub fn apply_offer_available(accounts: &ApplyOffer) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}

pub fn cancel_offer_available(accounts: &CancelOffer) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.offer.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}

pub fn accept_offer_available(accounts: &AcceptOffer) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    if accounts.offer.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}