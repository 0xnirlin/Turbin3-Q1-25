use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenInterface, Transfer, transfer},
    associated_token::AssociatedToken,
};
use crate::state::OTCOrderMaker;

// when creating the order we just want to take the sol from the buyer and send it to the vault that is controlled by this program. 

// no ata is needed since only the sol transfer is needed. 

#[derive(Accounts)]
#[instruction(amount: u64, seed: u64)]
pub struct CreateOTCOrder<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    
    // Using a seed parameter allows creating multiple orders with same amount
    #[account(
        init,
        payer = buyer,
        space = OTCOrderMaker::SIZE,
        seeds = [b"otc order", token_mint.key().as_ref(), buyer.key().as_ref(), &amount.to_le_bytes(), &seed.to_le_bytes()],
        bump,
    )]
    pub otc_order: Account<'info, OTCOrderMaker>,

    #[account(
        seeds = [b"listing", token_mint.key().as_ref(), maker.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"vault", token_mint.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, SystemAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CreateOTCOrder<'info> {
    pub fn create_otc_order(&mut self, amount: u64, seed: u64, bumps: &CreateOTCOrderBumps, seller: Option<Pubkey>) -> Result<()> {
        self.otc_order.set_inner(OTCOrderMaker {
            buyer: self.buyer.key(),
            token_mint: self.token_mint.key(),
            sol_amount: amount,
            seller: seller, // so this could be done or it could be someone
            bump: bumps.otc_order,
        });

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}