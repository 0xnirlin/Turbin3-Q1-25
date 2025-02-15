use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
pub struct Observation {
    // The block timestamp of the observation
    pub block_timestamp: u32,
    // The cumulative price at this timestamp
    pub price_cumulative: u64,
    // Padding for future upgrades
    pub padding: [u64; 4]
}

impl Observation {
    pub const SIZE: usize = 4 + // block_timestamp
                           8 + // price_cumulative 
                           32; // padding (4 * 8)

    pub fn new(block_timestamp: u32, price_cumulative: u64) -> Self {
        Observation {
            block_timestamp,
            price_cumulative,
            padding: [0u64; 4]
        }
    }
}

