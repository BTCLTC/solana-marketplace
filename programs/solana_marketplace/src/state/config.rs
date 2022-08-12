use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Default)]
pub struct Config {
    pub owner: Pubkey,
    pub fee_account: Pubkey,
    pub count_sells: u64,
    pub trade_fee_rate: u64, /* % */
    pub order_id: u64,
    pub freeze_program: bool,
    pub nonce: u8,
}

impl Config {
    pub const LEN: usize = (32 * 2) + (8 * 3) + 1 + 1;
}
