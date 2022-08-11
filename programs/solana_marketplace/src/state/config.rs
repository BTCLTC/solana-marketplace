use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Default)]
pub struct Config {
    pub owner: Pubkey,
    pub count_sells: u64,
    pub trade_fee_rate: u64, /* % */
    pub sell_id: u64,
    pub offer_id: u64,
    pub freeze_program: bool,
    pub nonce: u8,
}

impl Config {
    pub const LEN: usize = 32 + (8 * 4) + 1 + 1;
}
