use anchor_lang::prelude::*;

#[account]
// this is generalised config that is used to send the fee to the protocol treasury 
// now I want the premium such that the it can be set between certain range by the cretor - this config will ensure in each trade premium offered in between the range of 1-10% no more than that and no less than that

pub struct TurbineConfig {
    pub fee_percentage: u16,
    pub owner: Pubkey,
    pub max_fee_percentage: u16,
    pub min_fee_percentage: u16,
    pub treasury_bump: u8,
    pub bump: u8,
    pub max_premium: u16,
    pub min_premium: u16,
    pub listing_fee: u16,
}

impl TurbineConfig {
    pub const SIZE: usize = 8 + // discriminator
                           2 + // fee_percentage 
                           2 + // max_fee_percentage
                           2 + // min_fee_percentage
                           1 + // treasury_bump
                           1 + // bump
                           2 + // max_premium
                           2; // min_premium
}
