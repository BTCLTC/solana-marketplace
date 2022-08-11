use anchor_lang::prelude::*;
use crate::NAME_MAX_LEN;

#[account]
#[derive(Default)]
pub struct Config {
    pub nft_type: String,
    pub owner: Pubkey,
    pub count_sells: u64,
    pub trade_fee_rate: u64 /* % */,
    pub sell_id: u64,
    pub offer_id: u64,
    pub freeze_program: bool,
    pub usdc_mint: AcceptToken,
    pub sol_mint: AcceptToken,
    pub token1_mint: AcceptToken,
    pub token2_mint: AcceptToken,
    pub token3_mint: AcceptToken,
    pub nonce: u8,
}

impl Config {
    pub const LEN: usize = NAME_MAX_LEN + 32 + (8 * 4) + 1 + (40 * 5) + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AcceptToken {
    mint_key: Pubkey,
    decimals: u8,
}

impl AcceptToken {
    pub fn new(mint_key: Pubkey, decimals: u8) -> Self {
        AcceptToken { mint_key, decimals }
    }
    pub fn get_mint(&self) -> Pubkey {
        self.mint_key
    }
    pub fn get_decimals(&self) -> u8 {
        self.decimals
    }
}

#[account]
#[derive(Default)]
pub struct TokenConfig {
    pub nft_type: String,
    pub token_type: String,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub index: u8,
    pub fee: u64,
    pub freeze: bool,
    pub nonce: u8,
}

impl TokenConfig {
    pub const LEN: usize = NAME_MAX_LEN + NAME_MAX_LEN + 32 + 32 + 32 + 8 + 1 + 1 + 1;
}

#[account]
#[derive(Default)]
pub struct Sell {
    pub id: u64,
    pub nft_type: String,
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_vault: Pubkey,
    pub price: u64,
    pub flags: [u8; 5],
    pub offer_count: u64,
    pub created_at: u64,
}

impl Sell {
    pub const LEN: usize = 8 + NAME_MAX_LEN + (32 * 3) + 8 + 8 + 8 + 5;
}

#[account]
#[derive(Default)]
pub struct Offer {
    pub id: u64,
    pub nft_type: String,
    pub sell_id: u64,
    pub owner: Pubkey,
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub offer_price: u64,
    pub index: u8,
    pub expired_at: u64,
    pub created_at: u64,
}

impl Offer {
    pub const LEN: usize = 8 + NAME_MAX_LEN + 8 + 32 + 32 + 32 + 8 + 1 + 8 + 8;
}