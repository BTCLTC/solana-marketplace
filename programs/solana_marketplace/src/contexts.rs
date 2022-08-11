use crate::constants::*;
use crate::models::*;
use crate::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    init,
    payer = owner,
    seeds = [ CONFIG_PDA_SEED.as_ref()],
    bump,
    space = 8 + Config::LEN
    )]
    pub config: Box<Account<'info, Config>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    mut,
    seeds = [ CONFIG_PDA_SEED.as_ref()],
    bump,
    constraint = config.owner == owner.key() @ ErrorCode::PermissionError
    )]
    pub config: Box<Account<'info, Config>>,
}

#[derive(Accounts)]
pub struct ProgramFreeze<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    seeds = [ CONFIG_PDA_SEED.as_ref()],
    bump,
    constraint = config.owner == owner.key() @ ErrorCode::PermissionError
    )]
    pub config: Box<Account<'info, Config>>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct TokenSetUp<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    mut,
    constraint = config.owner == owner.key() @ ErrorCode::PermissionError,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    init,
    payer = owner,
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    space = 8 + TokenConfig::LEN
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct TokenFreeze<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    mut,
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    constraint = token_config.owner == owner.key() @ ErrorCode::PermissionError
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct InitTokenAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    seeds = [ CONFIG_PDA_SEED.as_ref()],
    bump,
    constraint = config.owner == owner.key() @ ErrorCode::PermissionError,
    )]
    pub config: Box<Account<'info, Config>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    init,
    payer = owner,
    token::mint = token_mint,
    token::authority = token_vault,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct StartSell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    has_one = token_mint,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
    init,
    payer = user,
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
    constraint = user_nft_vault.owner == user.key()
    )]
    pub user_nft_vault: Box<Account<'info, TokenAccount>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user_token_vault: UncheckedAccount<'info>,

    #[account(
    init,
    payer = user,
    seeds = [
    SELL_PDA_SEED.as_ref(),
    user.key().as_ref(),
    nft_mint.key().as_ref(),
    ],
    bump,
    space = 8 + Sell::LEN
    )]
    pub sell: Box<Account<'info, Sell>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct UpdateSell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    constraint = sell.owner == user.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    seeds = [
    SELL_PDA_SEED.as_ref(),
    user.key().as_ref(),
    nft_mint.key().as_ref(),
    ],
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct CloseSell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    constraint = nft_vault.mint == nft_mint.key(),
    seeds = [
    NFT_VAULT_PDA_SEED.as_ref(),
    nft_mint.key().as_ref()
    ],
    bump
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_nft_vault.mint == nft_mint.key(),
    constraint = user_nft_vault.owner == user.key()
    )]
    pub user_nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = sell.owner == user.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    constraint = sell.nft_vault == nft_vault.key(),
    seeds = [
    SELL_PDA_SEED.as_ref(),
    user.key().as_ref(),
    nft_mint.key().as_ref(),
    ],
    close = user,
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
    mut,
    constraint = seller.key() != buyer.key(),
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub seller: AccountInfo<'info>,

    #[account(
    mut,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    mut,
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    has_one = token_mint,
    has_one = token_vault,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    constraint = nft_vault.mint == nft_mint.key(),
    seeds = [
    NFT_VAULT_PDA_SEED.as_ref(),
    nft_mint.key().as_ref()
    ],
    bump
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = buyer_nft_vault.mint == nft_mint.key(),
    constraint = buyer_nft_vault.owner == buyer.key()
    )]
    pub buyer_nft_vault: Box<Account<'info, TokenAccount>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer_token_wallet: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub seller_token_wallet: UncheckedAccount<'info>,

    #[account(
    mut,
    constraint = sell.owner == seller.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    constraint = sell.nft_vault == nft_vault.key(),
    constraint = sell.token_type == _token_type,
    constraint = sell.owner_token_vault == seller_token_wallet.key(),
    seeds = [
    SELL_PDA_SEED.as_ref(),
    seller.key().as_ref(),
    nft_mint.key().as_ref(),
    ],
    close = seller,
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8, _sell_id: u64)]
pub struct ApplyOffer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
    mut,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    has_one = token_mint,
    has_one = token_vault,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer_token_wallet: UncheckedAccount<'info>,

    #[account(
    mut,
    constraint = sell.owner != buyer.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    constraint = sell.token_type == _token_type,
    seeds = [
    SELL_PDA_SEED.as_ref(),
    sell.owner.as_ref(),
    nft_mint.key().as_ref(),
    ],
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    #[account(
    init,
    payer = buyer,
    seeds = [
    OFFER_PDA_SEED.as_ref(),
    buyer.key().as_ref(),
    nft_mint.key().as_ref(),
    _sell_id.to_string().as_ref(),
    ],
    bump,
    space = 8 + Offer::LEN
    )]
    pub offer: Box<Account<'info, Offer>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8, _sell_id: u64)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    has_one = token_mint,
    has_one = token_vault,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer_token_wallet: UncheckedAccount<'info>,

    #[account(
    mut,
    constraint = sell.owner != buyer.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    constraint = sell.token_type == _token_type,
    seeds = [
    SELL_PDA_SEED.as_ref(),
    sell.owner.as_ref(),
    nft_mint.key().as_ref(),
    ],
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    #[account(
    mut,
    constraint = offer.sell_id == _sell_id,
    constraint = offer.owner == buyer.key(),
    constraint = offer.nft_mint == nft_mint.key(),
    seeds = [
    OFFER_PDA_SEED.as_ref(),
    buyer.key().as_ref(),
    nft_mint.key().as_ref(),
    _sell_id.to_string().as_ref(),
    ],
    close = buyer,
    bump,
    )]
    pub offer: Box<Account<'info, Offer>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_token_type: u8, _sell_id: u64)]
pub struct AcceptOffer<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
    mut,
    constraint = buyer.key() != seller.key(),
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: AccountInfo<'info>,

    #[account(
    mut,
    seeds = [CONFIG_PDA_SEED.as_ref()],
    bump = config.nonce
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    mut,
    seeds = [
    TOKEN_CONFIG_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump,
    has_one = token_mint,
    has_one = token_vault,
    )]
    pub token_config: Box<Account<'info, TokenConfig>>,

    #[account(
    constraint = nft_mint.supply == 1,
    constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    constraint = nft_vault.mint == nft_mint.key(),
    seeds = [
    NFT_VAULT_PDA_SEED.as_ref(),
    nft_mint.key().as_ref()
    ],
    bump
    )]
    pub nft_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = buyer_nft_vault.mint == nft_mint.key(),
    constraint = buyer_nft_vault.owner == buyer.key()
    )]
    pub buyer_nft_vault: Box<Account<'info, TokenAccount>>,

    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    mut,
    seeds = [
    TOKEN_VAULT_PDA_SEED.as_ref(),
    & [_token_type]
    ],
    bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub seller_token_wallet: UncheckedAccount<'info>,

    #[account(
    mut,
    constraint = sell.owner == seller.key(),
    constraint = sell.nft_mint == nft_mint.key(),
    constraint = sell.nft_vault == nft_vault.key(),
    constraint = sell.token_type == _token_type,
    constraint = sell.owner_token_vault == seller_token_wallet.key(),
    seeds = [
    SELL_PDA_SEED.as_ref(),
    seller.key().as_ref(),
    nft_mint.key().as_ref(),
    ],
    close = seller,
    bump
    )]
    pub sell: Box<Account<'info, Sell>>,

    #[account(
    mut,
    constraint = offer.sell_id == _sell_id,
    constraint = offer.owner == buyer.key(),
    constraint = offer.nft_mint == nft_mint.key(),
    seeds = [
    OFFER_PDA_SEED.as_ref(),
    buyer.key().as_ref(),
    nft_mint.key().as_ref(),
    _sell_id.to_string().as_ref(),
    ],
    close = buyer,
    bump,
    )]
    pub offer: Box<Account<'info, Offer>>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
