use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub maker: Pubkey,
    pub seed: u8, // seed used to derive the escrow vault this is very convenient I just realized
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

// so we define the tokens we want to exchange and how many tokens we want for what is inside the escrow vault