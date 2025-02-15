use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenInterface, Transfer, transfer},
    associated_token::AssociatedToken,
};
use crate::state::OTCOrderMaker;

// when taking the order we want to transfer the tokens from the seller to the buyer's ATA
// and transfer the SOL from the vault to the seller

#[derive(Accounts)]
pub struct TakeOTCOrder<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    // buyer will not be defined as signer
    pub buyer: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::authority = buyer,
        associated_token::mint = token_mint,
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)] 
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    // we are not closing this account since, we will allow partial fills if seller is set to NONE, otherwise it will be take it all or leave it, will create a separate instruction for closing the order
    #[account(
        mut,
        seeds = [b"otc order", token_mint.key().as_ref(), buyer.key().as_ref(), &otc_order.sol_amount.to_le_bytes()],
        bump = otc_order.bump,
    )]
    pub otc_order: Account<'info, OTCOrderMaker>,

    #[account(
        mut,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump = otc_order.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TakeOTCOrder<'info> {
    pub fn take_otc_order(&mut self, amount_sol: u64) -> Result<()> {
        // check if the amount of SOL is greater than the amount of SOL in the order
        if amount_sol > self.otc_order.sol_amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        // for the mint, fetch the token twap price from the raydium.

        // now calculate how much tokens user should send to get the amount of SoL he wants, applying the premium set by the maker.

        // now transfer the tokens from seller to buyer

        // now transfer the SOL from vault to seller
        
        
        
        Ok(())

    }
}
