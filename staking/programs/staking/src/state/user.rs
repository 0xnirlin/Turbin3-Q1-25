use anchor_lang::prelude::*;

// in a user struct we want to keep track of user bump, total staked by the user and last update timestamp
#[account]
pub struct User {
    pub bump: u8,
    pub total_staked: u32,
    pub last_update: u64,
}
