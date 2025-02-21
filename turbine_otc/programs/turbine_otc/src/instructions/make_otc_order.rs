/*
████████╗██╗   ██╗██████╗ ██████╗ ██╗███╗   ██╗███████╗     ██████╗ ████████╗ ██████╗
╚══██╔══╝██║   ██║██╔══██╗██╔══██╗██║████╗  ██║██╔════╝    ██╔═══██╗╚══██╔══╝██╔════╝
   ██║   ██║   ██║██████╔╝██████╔╝██║██╔██╗ ██║█████╗      ██║   ██║   ██║   ██║     
   ██║   ██║   ██║██╔══██╗██╔══██╗██║██║╚██╗██║██╔══╝      ██║   ██║   ██║   ██║     
   ██║   ╚██████╔╝██║  ██║██████╔╝██║██║ ╚████║███████╗    ╚██████╔╝   ██║   ╚██████╗
   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝     ╚═════╝    ╚═╝    ╚═════╝
*/

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::state::{OTCOrderMaker, TurbineConfig};
use solana_program::system_program;
use anchor_lang::system_program::{Transfer, transfer};


// ============================================================================
// This instruction handles the creation of an OTC (Over-The-Counter) order
// The buyer deposits SOL into a program-controlled vault as collateral
// The order can be either:
// 1. Open to any seller (seller = None)
// 2. Restricted to a specific seller (seller = Some(pubkey))
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Premium must be within allowed range")]
    InvalidPremium,
    #[msg("Invalid buyer")]
    InvalidBuyer,
}

#[derive(Accounts)]
#[instruction(amount: u64, seed: u64, expiry_timestamp: u64)]
pub struct CreateOTCOrder<'info> {
    // The buyer who initiates the OTC order and provides SOL collateral
    #[account(mut)]
    pub buyer: Signer<'info>,

    // The mint address of the token the buyer wishes to purchase
    pub token_mint: InterfaceAccount<'info, Mint>,

    
    // The OTC order PDA that stores all order details
    // Multiple orders can be created with same amount using different seeds
    #[account(
        init,
        payer = buyer,
        space = OTCOrderMaker::SIZE,
        seeds = [b"otc order", token_mint.key().as_ref(), buyer.key().as_ref(), &amount.to_le_bytes(), &seed.to_le_bytes()],
        bump,
    )]
    pub otc_order: Account<'info, OTCOrderMaker>,

    // Program-controlled vault that holds buyer's SOL collateral
    #[account(
        mut,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    // Config account that stores protocol parameters
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, TurbineConfig>,

    // Required program accounts
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateOTCOrder<'info> {
    pub fn create_otc_order(&mut self, amount: u64, seed: u64, bumps: CreateOTCOrderBumps, seller: Option<Pubkey>, expiry_timestamp: u64, premium: u16) -> Result<()> {

        require!(premium >= self.config.min_premium, ErrorCode::InvalidPremium);
        require!(premium <= self.config.max_premium, ErrorCode::InvalidPremium);

        // Initialize the OTC order account with buyer details, amount, and constraints
        self.otc_order.set_inner(OTCOrderMaker {
            buyer: self.buyer.key(),
            token_mint: self.token_mint.key(),
            sol_amount: amount,
            seller,
            bump: bumps.otc_order,
            vault_bump: bumps.vault,
            expiration_timestamp: expiry_timestamp,
            premium,
        });

        // Transfer SOL from buyer to program vault using checked transfer
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn cancel_otc_order(&mut self) -> Result<()> {
        // Only the buyer who created the order can cancel it
        require_keys_eq!(
            self.buyer.key(),
            self.otc_order.buyer,
            ErrorCode::InvalidBuyer
        );

        let token_mint = self.token_mint.key();
        // Transfer SOL back from vault to buyer
        let vault_seeds = &[
            b"vault",
            token_mint.as_ref(),
            &[self.otc_order.vault_bump],
        ];
        let vault_signer = &[&vault_seeds[..]];

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.buyer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, vault_signer);
        transfer(cpi_ctx, self.otc_order.sol_amount)?;

        Ok(())
    }
}