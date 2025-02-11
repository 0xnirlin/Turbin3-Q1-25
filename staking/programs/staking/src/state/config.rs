use anchor_lang::prelude::*;

// in a config we want the freeze period, max stake amount, bump 
#[account]
pub struct Config {
    pub freeze_period: u32,
    pub max_stake: u32,
    pub bump: u8,
    pub collection_key: Pubkey,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 4 + 4 + 1;
}
