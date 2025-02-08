use anchor_lang::prelude::*;

#[account]
pub struct Config {
    // I will try to do it without the authorityu
    pub initializer: Pubkey,
    pub authority: Option<Pubkey>, //authority is optional.
    pub fee: u16,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub locked: bool,
    pub lp_bump: u8, // the lp token that we will mint to the user when they deposit
    pub config_bump: u8,
    pub seed: String,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 2 + 32 + 32 + 8 + 1 + 1;
}
