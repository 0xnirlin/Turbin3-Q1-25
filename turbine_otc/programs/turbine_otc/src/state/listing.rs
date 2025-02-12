use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub initiator: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}

// at the listinng level we probably not want anything, fee is handled at the config level and the creator of listing will be permissionless so we don't want to grant them any permissions here

impl Listing {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // initiator
                           32 + // mint
                           1 + // bump
                           2; // listing_fee
}