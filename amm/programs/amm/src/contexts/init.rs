use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        Mint, TokenAccount, TokenInterface
    },
    associated_token::AssociatedToken
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(
    seed: String,
)]
pub struct Init<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // this will be the creator of pool 

    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = token_a_mint,
        associated_token::authority = config,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = token_b_mint,
        associated_token::authority = config,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    // these seeds ensure that lp can be derived from the config - config is derived from two tokens and their seed provided
    // the logic for seed is that we can create multiple pools for same tokens with different configurations
    #[account(
        init,
        payer = signer,
        seeds = [b"lp", config.key().as_ref()],
        mint::decimals = 6,
        mint::authority = config,
        bump,
    )]
    pub lp_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = Config::INIT_SPACE,
        seeds = [b"config",token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), seed.as_bytes()],
        bump,
    )]
    pub config: Account<'info, Config>, //  the config that we will store about the pool

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> Init<'info> {
    pub fn init(&mut self, fee: u16, authority: Option<Pubkey>, seed: String, bumps: &InitBumps) -> Result<()> {
        // initially locked will be false
        self.config.set_inner(Config {
            initializer: self.signer.key(),
            authority,
            fee,
            token_a_mint: self.token_a_mint.key(),
            token_b_mint: self.token_b_mint.key(),
            locked: false,
            lp_bump: bumps.lp_mint,
            config_bump: bumps.config,
            seed,
        });
        Ok(())
    }
}
