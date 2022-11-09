use anchor_lang::{prelude::*, system_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount, PREFIX};
use solana_program::sysvar::rent;

use crate::{
    constants::{CONFIG_PDA_SEED, NFT_VAULT_PDA_SEED, SELL_PDA_SEED},
    errors::ErrorCode,
    states::{Config, Sell},
    validate::verify_metadata,
};

#[event]
pub struct SellEvent {
    order_id: u64,
    seller: Pubkey,
    nft_mint: Pubkey,
    nft_vault: Pubkey,
    price: u64,
    created_at: u64,
}

#[derive(Accounts)]
pub struct SellNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED.as_ref()],
        bump = config.load()?.bump
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(
        constraint = nft_mint.supply == 1,
        constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [PREFIX.as_bytes(), mpl_token_metadata::ID.as_ref(), nft_mint.key().as_ref()],
        seeds::program = mpl_token_metadata::ID,
        bump
    )]
    pub nft_metadata: AccountInfo<'info>,

    #[account(
        init,
        payer = seller,
        token::mint = nft_mint,
        token::authority = nft_vault,
        seeds = [
            NFT_VAULT_PDA_SEED.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_nft_vault.mint == nft_mint.key(),
        constraint = user_nft_vault.owner == seller.key()
    )]
    pub user_nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = seller,
        seeds = [
            SELL_PDA_SEED.as_ref(),
            seller.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump,
        space = 16 + Sell::LEN
    )]
    pub sell: AccountLoader<'info, Sell>,

    /// used by anchor for init of the token
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn sell_nft_handle(ctx: Context<SellNFT>, price: u64) -> Result<()> {
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let mut config = ctx.accounts.config.load_mut()?;
    let mut sell = ctx.accounts.sell.load_init()?;

    require!(price > 0, ErrorCode::InvalidTokenAmount);

    let metadata: Metadata =
        Metadata::from_account_info(&ctx.accounts.nft_metadata.to_account_info())?;

    if let Some(creators) = &metadata.data.creators {
        verify_metadata(creators)?;
    }

    // LOCK NFT : Transfer nft to nft_vault PDA
    {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_nft_vault.to_account_info(),
                to: ctx.accounts.nft_vault.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, 1)?;
    }

    // Save Sell info
    let order_id = config.order_id;
    sell.order_id = config.order_id;
    sell.seller = ctx.accounts.seller.key();
    sell.nft_mint = ctx.accounts.nft_mint.key();
    sell.nft_vault = ctx.accounts.nft_vault.key();
    sell.price = price;
    sell.created_at = now_ts as u64;
    sell.updated_at = now_ts as u64;

    // Update config
    config.order_count += 1;
    config.order_id += 1;

    // sell event
    emit!(SellEvent {
        order_id: order_id,
        seller: ctx.accounts.seller.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        nft_vault: ctx.accounts.nft_vault.key(),
        price,
        created_at: now_ts as u64,
    });

    Ok(())
}
