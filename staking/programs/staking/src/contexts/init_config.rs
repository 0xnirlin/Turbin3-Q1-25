use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::state::Config;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
        payer = payer,
        space = Config::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,


    pub system_program: Program<'info, System>,
}


impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self, freeze_period: u32, max_stake: u32, collection_key: Pubkey, bumps: &InitConfigBumps) {
        // require collection key is not already set
        require!(self.config.collection_key == Pubkey::default(), StakingError::CollectionKeyAlreadySet);

        self.config.set_inner(Config {
            collection_key,
            freeze_period,
            max_stake,
            bump: bumps.config,
        });
        Ok(())
    }
}


#[error_code]
pub enum StakingError {
    #[msg("Collection key already set")]
    CollectionKeyAlreadySet,
}