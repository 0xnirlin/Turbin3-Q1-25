use anchor_lang::prelude::*;

#[account]
pub struct OTCOrderTaker {
    pub buyer: Pubkey,
    pub token_mint: Pubkey,
}

impl OTCOrderTaker {
    pub const SIZE: usize = 8 + 32 + 32;
}