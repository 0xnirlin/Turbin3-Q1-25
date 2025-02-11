use anchor_lang::prelude::*;


// stake account has the mint, bump

// the purpose of stake account is to control each nft. 
// and we need to keep track of the stake account for each user.

#[account]
pub struct Stake {
    pub mint: Pubkey,
    pub last_update: u64,
    pub bump: u8,
}