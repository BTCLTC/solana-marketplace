use crate::{errors::ErrorCode, BuyNFT, CloseSell, SellNFT, UpdateSell};
use anchor_lang::prelude::*;
use mpl_token_metadata::state::Creator;

pub fn sell_nft_available(accounts: &SellNFT) -> Result<()> {
    let config = accounts.config.load()?;

    if config.freeze {
        return err!(ErrorCode::FreezeProgramError);
    }

    Ok(())
}

pub fn update_sell_available(accounts: &UpdateSell) -> Result<()> {
    let config = accounts.config.load()?;
    let sell = accounts.sell.load()?;

    if config.freeze {
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

    if config.freeze {
        return err!(ErrorCode::FreezeProgramError);
    }
    if sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}

pub fn buy_available(accounts: &BuyNFT) -> Result<()> {
    let config = accounts.config.load()?;
    let sell = accounts.sell.load()?;

    if config.freeze {
        return err!(ErrorCode::FreezeProgramError);
    }
    if sell.created_at == 0 {
        return err!(ErrorCode::InvalidRequestError);
    }

    Ok(())
}

pub fn verify_metadata(creators: &[Creator]) -> Result<()> {
    let sum: u8 = creators.iter().map(|x| x.share).sum();

    if sum != 100 {
        return err!(ErrorCode::InvalidSharesSum);
    }

    Ok(())
}
