use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Default)]
pub struct Sell {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_vault: Pubkey,
    pub order_id: u64,
    pub price: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Sell {
    pub const LEN: usize = 8 + (32 * 3) + (8 * 3);
}
