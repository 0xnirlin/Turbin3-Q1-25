use anchor_lang::prelude::*;

use crate::state::TurbineConfig;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = TurbineConfig::SIZE,
        seeds = [b"turbine_config"],
        bump,
    )]
    pub config: Account<'info, TurbineConfig>,
    #[account(
        seeds = [b"treasury", config.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn init(&mut self, fee_percentage: u16, max_fee_percentage: u16, min_fee_percentage: u16, max_premium: u16, min_premium: u16, bumps: InitConfigBumps, owner: Pubkey, listing_fee: u16) -> Result<()> {
        // require the creator to be a specific hardcoded wallet address
        require_eq!(
            self.creator.key(),
            pubkey!("HXtBm8XZbxaTt41uqaKhwUAa6Z1aPyvJGE1111111111"),
            TurbineError::InvalidOwner
        );
        // require the fee percentage to be between the max and min fee percentage
        require!(fee_percentage <= max_fee_percentage && fee_percentage >= min_fee_percentage, TurbineError::InvalidFeePercentage);
        // require the premium to be between the max and min premium
        require!(max_premium >= min_premium, TurbineError::InvalidPremium);
        
        self.config.set_inner(TurbineConfig {
            fee_percentage,
            max_fee_percentage,
            min_fee_percentage,
            max_premium,
            min_premium,
            owner,
            treasury_bump: bumps.treasury,
            bump: bumps.config,
            listing_fee,
        });
        Ok(())
    }

    pub fn set_max_fee_percentage(&mut self, max_fee_percentage: u16) -> Result<()> {
        require_eq!(self.creator.key(), self.config.owner, TurbineError::InvalidOwner);
        require!(max_fee_percentage >= self.config.fee_percentage, TurbineError::InvalidMaxFeePercentage);
        self.config.max_fee_percentage = max_fee_percentage;
        Ok(())
    }

    pub fn set_min_fee_percentage(&mut self, min_fee_percentage: u16) -> Result<()> {
        require_eq!(self.creator.key(), self.config.owner, TurbineError::InvalidOwner);
        require!(min_fee_percentage <= self.config.fee_percentage, TurbineError::InvalidMinFeePercentage);
        self.config.min_fee_percentage = min_fee_percentage;
        Ok(())
    }

    pub fn set_max_premium(&mut self, max_premium: u16) -> Result<()> {
        require_eq!(self.creator.key(), self.config.owner, TurbineError::InvalidOwner);
        require!(max_premium >= self.config.min_premium, TurbineError::InvalidMaxPremium);
        self.config.max_premium = max_premium;
        Ok(())
    }

    pub fn set_min_premium(&mut self, min_premium: u16) -> Result<()> {
        require_eq!(self.creator.key(), self.config.owner, TurbineError::InvalidOwner);
        require!(min_premium <= self.config.max_premium, TurbineError::InvalidMinPremium);
        self.config.min_premium = min_premium;
        Ok(())
    }
    
}

#[error_code]
pub enum TurbineError {
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,
    #[msg("Invalid premium")]
    InvalidPremium,
    #[msg("Invalid max fee percentage")]
    InvalidMaxFeePercentage,
    #[msg("Invalid min fee percentage")]
    InvalidMinFeePercentage,
    #[msg("Invalid max premium")]
    InvalidMaxPremium,
    #[msg("Invalid min premium")]
    InvalidMinPremium,
}