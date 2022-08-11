use anchor_lang::prelude::*;

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
