// Access control modifier
use anchor_lang::prelude::*;
use crate::{StartSell, ErrorCode, UpdateSell, CloseSell, Buy, ApplyOffer, CancelOffer, AcceptOffer};

pub fn start_sell_available(accounts: &StartSell) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    Ok(())
}

pub fn update_sell_available(accounts: &UpdateSell) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
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
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }
    Ok(())
}

pub fn buy_available(accounts: &Buy, index: &u8) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    match index {
        1 => {
            let usdc = accounts.config.usdc_mint.clone();
            if usdc.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[0] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        2 => {
            let sol = accounts.config.sol_mint.clone();
            if sol.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[1] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        3 => {
            let token1 = accounts.config.token1_mint.clone();
            if token1.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[2] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        4 => {
            let token2 = accounts.config.token2_mint.clone();
            if token2.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[3] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        5 => {
            let token3 = accounts.config.token3_mint.clone();
            if token3.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[4] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        _ => return err!(ErrorCode::InvalidTokenMint)
    }

    Ok(())
}

pub fn apply_offer_available(accounts: &ApplyOffer, index: &u8) -> Result<()> {
    if accounts.config.freeze_program {
        return err!(ErrorCode::FreezeProgramError);
    }
    if accounts.token_config.freeze {
        return err!(ErrorCode::FreezeTokenError);
    }
    if accounts.sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    match index {
        1 => {
            let usdc = accounts.config.usdc_mint.clone();
            if usdc.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[0] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        2 => {
            let sol = accounts.config.sol_mint.clone();
            if sol.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[1] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        3 => {
            let token1 = accounts.config.token1_mint.clone();
            if token1.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[2] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        4 => {
            let token2 = accounts.config.token2_mint.clone();
            if token2.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[3] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        5 => {
            let token3 = accounts.config.token3_mint.clone();
            if token3.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[4] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        _ => return err!(ErrorCode::InvalidTokenMint)
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

    match accounts.offer.index {
        1 => {
            let usdc = accounts.config.usdc_mint.clone();
            if usdc.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[0] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        2 => {
            let sol = accounts.config.sol_mint.clone();
            if sol.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[1] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        3 => {
            let token1 = accounts.config.token1_mint.clone();
            if token1.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[2] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        4 => {
            let token2 = accounts.config.token2_mint.clone();
            if token2.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[3] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        5 => {
            let token3 = accounts.config.token3_mint.clone();
            if token3.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[4] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        _ => return err!(ErrorCode::InvalidTokenMint)
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

    match accounts.offer.index {
        1 => {
            let usdc = accounts.config.usdc_mint.clone();
            if usdc.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[0] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        2 => {
            let sol = accounts.config.sol_mint.clone();
            if sol.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[1] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        3 => {
            let token1 = accounts.config.token1_mint.clone();
            if token1.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[2] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        4 => {
            let token2 = accounts.config.token2_mint.clone();
            if token2.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[3] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        5 => {
            let token3 = accounts.config.token3_mint.clone();
            if token3.get_mint() != accounts.token_mint.key() {
                return err!(ErrorCode::InvalidTokenMint);
            }
            if accounts.sell.flags[4] != 1 {
                return err!(ErrorCode::InvalidTokenMintNotAllowed);
            }
        }
        _ => return err!(ErrorCode::InvalidTokenMint)
    }
    Ok(())
}