pub mod constants;
pub mod contexts;
pub mod validate;
pub mod models;
pub mod utils;

use crate::contexts::*;
use crate::validate::*;
use crate::utils::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction::transfer;

use crate::{constants::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_marketplace {
    use super::*;

    pub fn setup(
        ctx: Context<Setup>,
        _nonce_config: u8,
        trade_fee_rate: u64,
    ) -> Result<()> {
        msg!("Set up");
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.owner.key();
        config.trade_fee_rate = trade_fee_rate;
        config.sell_id = 1;
        config.offer_id = 1;
        config.nonce = _nonce_config;
        Ok(())
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        trade_fee_rate: u64,
    ) -> Result<()> {
        msg!("Update Config");
        let config = &mut ctx.accounts.config;
        config.trade_fee_rate = trade_fee_rate;
        Ok(())
    }

    pub fn toggle_freeze_program(
        ctx: Context<ProgramFreeze>,
    ) -> Result<()> {
        msg!("Toggle Freeze Program");
        let config = &mut ctx.accounts.config;
        config.freeze_program = !config.freeze_program;

        Ok(())
    }

    pub fn init_token_account(
        _ctx: Context<InitTokenAccount>,
        _token_type: u8,
    ) -> Result<()> {
        msg!("Init Set up TokenConfig");
        Ok(())
    }

    pub fn token_setup(
        ctx: Context<TokenSetUp>,
        _token_type: u8,
        _nonce: u8,
    ) -> Result<()> {
        msg!("Set up TokenConfig");
        let token_config = &mut ctx.accounts.token_config;

        token_config.owner = ctx.accounts.owner.key();
        token_config.token_type = _token_type;
        token_config.token_mint = ctx.accounts.token_mint.key();
        token_config.token_vault = ctx.accounts.token_vault.key();
        token_config.nonce = _nonce;

        Ok(())
    }

    pub fn toggle_freeze_token(
        ctx: Context<TokenFreeze>,
        _token_type: u8,
    ) -> Result<()> {
        msg!("Toggle Freeze Token");
        let token_config = &mut ctx.accounts.token_config;
        token_config.freeze = !token_config.freeze;

        Ok(())
    }

    #[access_control(start_sell_available(& ctx.accounts))]
    pub fn start_sell(
        ctx: Context<StartSell>,
        _token_type: u8,
        price: u64,
    ) -> Result<()> {
        msg!("Start Sell");
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let config = &mut ctx.accounts.config;
        let sell = &mut ctx.accounts.sell;

        require!(price > 0, ErrorCode::InvalidTokenAmount);

        // LOCK NFT : Transfer nft to nft_vault PDA
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_nft_vault.to_account_info(),
                    to: ctx.accounts.nft_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            );
            token::transfer(cpi_ctx, 1)?;
        }

        let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
        if is_native {
            assert_keys_equal(ctx.accounts.user.key(), ctx.accounts.user_token_vault.key())?;
        } else {
            let user_token_vault_mint = get_mint_from_token_account(&ctx.accounts.user_token_vault)?;
            let user_token_vault_owner = get_owner_from_token_account(&ctx.accounts.user_token_vault)?;
            assert_keys_equal(ctx.accounts.token_mint.key(), user_token_vault_mint)?;
            assert_keys_equal(ctx.accounts.user.key(), user_token_vault_owner)?;
        }

        // Save Sell info
        sell.id = config.sell_id;
        sell.owner = ctx.accounts.user.key();
        sell.owner_token_vault = ctx.accounts.user_token_vault.key();
        sell.nft_mint = ctx.accounts.nft_mint.key();
        sell.nft_vault = ctx.accounts.nft_vault.key();
        sell.price = price;
        sell.token_type = _token_type;
        sell.created_at = now_ts as u64;

        // Update config
        config.count_sells += 1;
        config.sell_id += 1;
        Ok(())
    }

    #[access_control(update_sell_available(& ctx.accounts))]
    pub fn update_sell(
        ctx: Context<UpdateSell>,
        _token_type: u8,
        price: u64,
    ) -> Result<()> {
        msg!("Update Sell");

        require!(price > 0, ErrorCode::InvalidTokenAmount);

        let sell = &mut ctx.accounts.sell;
        sell.price = price;

        Ok(())
    }

    #[access_control(close_sell_available(& ctx.accounts))]
    pub fn close_sell(
        ctx: Context<CloseSell>,
        _token_type: u8,
    ) -> Result<()> {
        msg!("Close Sell");
        let config = &mut ctx.accounts.config;
        // Transfer nft to user from vault
        {
            let nft_vault_bump = *ctx.bumps.get("nft_vault").unwrap();
            let seeds = &[
                NFT_VAULT_PDA_SEED.as_ref(),
                ctx.accounts.nft_mint.to_account_info().key.as_ref(),
                &[nft_vault_bump]
            ];
            let signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.nft_vault.to_account_info(),
                    to: ctx.accounts.user_nft_vault.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, 1)?;

            // Close nft vault
            let cpi_close_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::CloseAccount {
                    account: ctx.accounts.nft_vault.to_account_info(),
                    destination: ctx.accounts.user.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::close_account(cpi_close_ctx)?;
        }

        //Update config info
        config.count_sells -= 1;
        Ok(())
    }

    #[access_control(buy_available(& ctx.accounts))]
    pub fn buy(
        ctx: Context<Buy>,
        _token_type: u8,
    ) -> Result<()> {
        msg!("Buy");
        let config = &mut ctx.accounts.config;
        let token_config = &mut ctx.accounts.token_config;
        let sell = &mut ctx.accounts.sell;

        // Payment
        let fee: u64 = (sell.price as u128)
            .checked_mul(config.trade_fee_rate as u128)
            .unwrap()
            .checked_div(100)
            .unwrap()
            .try_into()
            .unwrap();

        let price: u64 = (sell.price as u128)
            .checked_sub(fee as u128)
            .unwrap()
            .try_into()
            .unwrap();

        let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
        if is_native {
            assert_keys_equal(ctx.accounts.buyer.key(), ctx.accounts.buyer_token_wallet.key())?;
            assert_keys_equal(ctx.accounts.seller.key(), ctx.accounts.seller_token_wallet.key())?;
            // send lamports to seller
            invoke(
                &transfer(
                    ctx.accounts.buyer_token_wallet.to_account_info().key,
                    ctx.accounts.seller_token_wallet.to_account_info().key,
                    price,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.seller.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
            )?;

            // send lamports to fee_vault
            invoke(
                &transfer(
                    ctx.accounts.buyer_token_wallet.to_account_info().key,
                    ctx.accounts.token_vault.to_account_info().key,
                    fee,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.token_vault.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
            )?;
        } else {
            // send fee token to token_vault
            let buyer_token_mint = get_mint_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            let seller_token_mint = get_mint_from_token_account(&ctx.accounts.seller_token_wallet)?;
            let buyer_token_owner = get_owner_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            let seller_token_owner = get_owner_from_token_account(&ctx.accounts.seller_token_wallet)?;

            assert_keys_equal(ctx.accounts.token_mint.key(), buyer_token_mint)?;
            assert_keys_equal(ctx.accounts.token_mint.key(), seller_token_mint)?;
            assert_keys_equal(ctx.accounts.buyer.key(), buyer_token_owner)?;
            assert_keys_equal(ctx.accounts.seller.key(), seller_token_owner)?;

            let cpi_fee_program = ctx.accounts.token_program.to_account_info();
            let cpi_price_program = ctx.accounts.token_program.to_account_info();
            let cpi_fee_accounts = Transfer {
                from: ctx.accounts.buyer_token_wallet.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_fee_ctx = CpiContext::new(cpi_fee_program, cpi_fee_accounts);
            token::transfer(cpi_fee_ctx, fee)?;

            let cpi_price_accounts = Transfer {
                from: ctx.accounts.buyer_token_wallet.to_account_info(),
                to: ctx.accounts.seller_token_wallet.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_price_ctx = CpiContext::new(cpi_price_program, cpi_price_accounts);
            token::transfer(cpi_price_ctx, price)?;
        }

        // Transfer nft to user from vault
        {
            let nft_vault_bump = *ctx.bumps.get("nft_vault").unwrap();
            let seeds = &[
                NFT_VAULT_PDA_SEED.as_ref(),
                ctx.accounts.nft_mint.to_account_info().key.as_ref(),
                &[nft_vault_bump]
            ];
            let signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.nft_vault.to_account_info(),
                    to: ctx.accounts.buyer_nft_vault.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, 1)?;

            // Close nft vault
            let cpi_close_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::CloseAccount {
                    account: ctx.accounts.nft_vault.to_account_info(),
                    destination: ctx.accounts.seller.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::close_account(cpi_close_ctx)?;
        }

        //Update token config
        token_config.fee += fee;

        //Update config info
        config.count_sells -= 1;
        Ok(())
    }

    #[access_control(apply_offer_available(& ctx.accounts))]
    pub fn apply_offer(
        ctx: Context<ApplyOffer>,
        _token_type: u8,
        _sell_id: u64,
        price: u64,
    ) -> Result<()> {
        msg!("Apply Offer");
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let config = &mut ctx.accounts.config;
        let sell = &mut ctx.accounts.sell;
        let offer = &mut ctx.accounts.offer;

        require!(price > 0, ErrorCode::InvalidTokenAmount);

        let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
        if is_native {
            // lock lamports to token_vault
            assert_keys_equal(ctx.accounts.buyer.key(), ctx.accounts.buyer_token_wallet.key())?;
            invoke(
                &transfer(
                    ctx.accounts.buyer_token_wallet.to_account_info().key,
                    ctx.accounts.token_vault.to_account_info().key,
                    price,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.token_vault.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
            )?;
        } else {
            // Lock token to token_vault
            let buyer_token_mint = get_mint_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            let buyer_token_owner = get_owner_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            assert_keys_equal(ctx.accounts.token_mint.key(), buyer_token_mint)?;
            assert_keys_equal(ctx.accounts.buyer.key(), buyer_token_owner)?;

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_accounts = Transfer {
                from: ctx.accounts.buyer_token_wallet.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, price)?;
        }

        // Save offer info
        offer.id = config.offer_id;
        offer.sell_id = sell.id;
        offer.owner = ctx.accounts.buyer.key();
        offer.seller = sell.owner;
        offer.nft_mint = ctx.accounts.nft_mint.key();
        offer.offer_price = price;
        offer.price_type = _token_type;
        offer.created_at = now_ts as u64;

        // update sell info
        sell.offer_count += 1;

        // update config info
        config.offer_id += 1;

        Ok(())
    }

    #[access_control(cancel_offer_available(& ctx.accounts))]
    pub fn cancel_offer(
        ctx: Context<CancelOffer>,
        _token_type: u8,
        _sell_id: u64,
    ) -> Result<()> {
        msg!("Cancel Offer");
        let sell = &mut ctx.accounts.sell;
        let offer = &mut ctx.accounts.offer;

        let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
        let token_vault_bump = *ctx.bumps.get("token_vault").unwrap();
        let token_vault_seeds = [TOKEN_VAULT_PDA_SEED.as_ref(), &[_token_type], &[token_vault_bump]];
        if is_native {
            assert_keys_equal(ctx.accounts.buyer.key(), ctx.accounts.buyer_token_wallet.key())?;
            invoke_signed(
                &transfer(
                    ctx.accounts.token_vault.key,
                    ctx.accounts.buyer_token_wallet.key,
                    offer.offer_price,
                ),
                &[
                    ctx.accounts.token_vault.to_account_info(),
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
                &[&token_vault_seeds],
            )?;
        } else {
            let buyer_token_mint = get_mint_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            let buyer_token_owner = get_owner_from_token_account(&ctx.accounts.buyer_token_wallet)?;
            assert_keys_equal(ctx.accounts.token_mint.key(), buyer_token_mint)?;
            assert_keys_equal(ctx.accounts.buyer.key(), buyer_token_owner)?;

            let signer = &[&token_vault_seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.buyer_token_wallet.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, offer.offer_price)?;
        }

        if sell.created_at != 0 {
            sell.offer_count -= 1;
        }

        Ok(())
    }

    #[access_control(accept_offer_available(& ctx.accounts))]
    pub fn accept_offer(
        ctx: Context<AcceptOffer>,
        _token_type: u8,
        _sell_id: u64,
    ) -> Result<()> {
        msg!("Accept Offer");
        let config = &mut ctx.accounts.config;
        let token_config = &mut ctx.accounts.token_config;
        let offer = &mut ctx.accounts.offer;

        // Payment
        let fee: u64 = (offer.offer_price as u128)
            .checked_mul(config.trade_fee_rate as u128)
            .unwrap()
            .checked_div(100)
            .unwrap()
            .try_into()
            .unwrap();

        let price: u64 = (offer.offer_price as u128)
            .checked_sub(fee as u128)
            .unwrap()
            .try_into()
            .unwrap();

        let token_vault_bump = *ctx.bumps.get("token_vault").unwrap();
        let token_vault_seeds = [TOKEN_VAULT_PDA_SEED.as_ref(), &[_token_type], &[token_vault_bump]];

        let is_native = ctx.accounts.token_mint.key() == spl_token::native_mint::id();
        if is_native {
            assert_keys_equal(ctx.accounts.seller.key(), ctx.accounts.seller_token_wallet.key())?;
            // send lamports to seller
            invoke_signed(
                &transfer(
                    ctx.accounts.token_vault.key,
                    ctx.accounts.seller_token_wallet.key,
                    price,
                ),
                &[
                    ctx.accounts.token_vault.to_account_info(),
                    ctx.accounts.seller.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],
                &[&token_vault_seeds],
            )?;
        } else {
            let seller_token_mint = get_mint_from_token_account(&ctx.accounts.seller_token_wallet)?;
            let seller_token_owner = get_owner_from_token_account(&ctx.accounts.seller_token_wallet)?;

            assert_keys_equal(ctx.accounts.token_mint.key(), seller_token_mint)?;
            assert_keys_equal(ctx.accounts.seller.key(), seller_token_owner)?;

            let signer = &[&token_vault_seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.seller_token_wallet.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, price)?;
        }

        // Transfer nft to user from vault
        {
            let nft_vault_bump = *ctx.bumps.get("nft_vault").unwrap();
            let seeds = &[
                NFT_VAULT_PDA_SEED.as_ref(),
                ctx.accounts.nft_mint.to_account_info().key.as_ref(),
                &[nft_vault_bump]
            ];
            let signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.nft_vault.to_account_info(),
                    to: ctx.accounts.buyer_nft_vault.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, 1)?;

            // Close nft vault
            let cpi_close_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::CloseAccount {
                    account: ctx.accounts.nft_vault.to_account_info(),
                    destination: ctx.accounts.seller.to_account_info(),
                    authority: ctx.accounts.nft_vault.to_account_info(),
                },
                signer,
            );
            token::close_account(cpi_close_ctx)?;
        }

        // Update token config
        token_config.fee += fee;

        // update config info
        config.count_sells -= 1;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Permission Error, E1000")]
    PermissionError,
    #[msg("The contract frozen, E1001")]
    FreezeProgramError,
    #[msg("The token frozen, E1002")]
    FreezeTokenError,
    #[msg("NFT Locked, E1003")]
    NFTLockedError,
    #[msg("Invalid Request, E1004")]
    InvalidRequestError,
    #[msg("Trade not available, E1005")]
    TradeNotAvailableError,
    #[msg("Not exist member, E1006")]
    NoMemberError,
    #[msg("Not enough SOL, E1007")]
    InsufficientSolAmountError,
    #[msg("Not enough Token, E1008")]
    InsufficientTokenAmountError,
    #[msg("The amount is small than min price, E1009")]
    InsufficientMinAmountError,
    #[msg("IncorrectOwner, E1010")]
    IncorrectOwner,
    #[msg("Derived key invalid, E1011")]
    DerivedKeyInvalid,
    #[msg("Metadata doesn't exist, E1012")]
    MetadataDoesntExist,
    #[msg("PublicKeyMismatch, E1013")]
    PublicKeyMismatch,
    #[msg("UninitializedAccount, E1014")]
    UninitializedAccount,
    #[msg("No payer present on this txn, E1015")]
    NoPayerPresent,
    #[msg("Invalid token amount, E1016")]
    InvalidTokenAmount,
}