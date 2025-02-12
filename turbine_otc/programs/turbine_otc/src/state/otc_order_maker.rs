use anchor_lang::prelude::*;

#[account]
// so in otc order we must also specifc for how much sol we are willing to buy the tokens and other calculation will be done using the raydium twap calculation
pub struct OTCOrderMaker {
    pub buyer: Pubkey,
    pub token_mint: Pubkey, // this is the token we are willing to buy
    pub sol_amount: u64,
    pub seller: Option<Pubkey>,
    pub bump: u8,
}

impl OTCOrderMaker {
    pub const SIZE: usize = 8 + // discriminator
                           32 + // buyer pubkey
                           32 + // token_mint pubkey
                           8 + // sol_amount
                           (1 + 32) + // Option<Pubkey> for seller
                           1; // bump
}
