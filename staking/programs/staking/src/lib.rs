use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_interface::{Mint, TokenAccount},
};

declare_id!("FpE8GXycapfGX4dDPLLUM9bSuidwymAN3RmPV1cMEx2c");

pub mod contexts;
pub mod state;

use crate::contexts::*;
use crate::state::{Config, User, Stake};

#[program]
pub mod staking {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, freeze_period: u32, max_stake: u32, collection_key: Pubkey) -> Result<()> {
        InitConfig::init_config(ctx, freeze_period, max_stake, collection_key)
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        InitUser::init_user(ctx);
        Ok(())
    }

    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        Stake::stake(ctx)
    }
}

#[error_code]
pub enum StakingError {
    #[msg("Collection key already set")]
    CollectionKeyAlreadySet,
    #[msg("Stake amount too high")]
    StakeAmountTooHigh,
}
