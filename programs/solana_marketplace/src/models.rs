use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Config {
    pub owner: Pubkey,
    pub count_sells: u64,
    pub trade_fee_rate: u64 /* % */,
    pub sell_id: u64,
    pub offer_id: u64,
    pub freeze_program: bool,
    pub nonce: u8,
}

impl Config {
    pub const LEN: usize = 32 + (8 * 4) + 1 + 1;
}

#[account]
#[derive(Default)]
pub struct TokenConfig {
    pub owner: Pubkey,
    pub token_type: u8,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub fee: u64,
    pub freeze: bool,
    pub nonce: u8,
}

impl TokenConfig {
    pub const LEN: usize = 32 + 1 + 32 + 32 + 8 + 1 + 1;
}

#[account]
#[derive(Default)]
pub struct Sell {
    pub id: u64,
    pub owner: Pubkey,
    pub owner_token_vault: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_vault: Pubkey,
    pub price: u64,
    pub token_type: u8,
    pub offer_count: u64,
    pub created_at: u64,
}

impl Sell {
    pub const LEN: usize = 8 + (32 * 4) + 8 + 1 + 8 + 8;
}

#[account]
#[derive(Default)]
pub struct Offer {
    pub id: u64,
    pub sell_id: u64,
    pub owner: Pubkey,
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub offer_price: u64,
    pub price_type: u8,
    pub created_at: u64,
}

impl Offer {
    pub const LEN: usize = 8 + 8 + 32 + 32 + 32 + 8 + 1 + 8;
}