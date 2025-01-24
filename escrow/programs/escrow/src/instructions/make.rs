use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
};
use crate::state::escrow::Escrow;


#[derive(Accounts)]
#[instruction(seeds: u8)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // first account is the signer
    // and we need escrow to put tokens into 
    // interface to deal with both kind of account
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // we have the mints for the tokens we want the token accounts too
    #[account(mut, associated_token::mint = mint_a, associated_token::authority = maker)]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,

    // we don't need the taker token account for now
    #[account(init, payer = maker, space = 8 + Escrow::INIT_SPACE, seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    // we need the vault to put the tokens into
    #[account(init, payer = maker, associated_token::mint = mint_b, associated_token::authority = escrow)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>, // we need it for token program actually
    pub token_program: Interface<'info, TokenInterface>, // we need it for mint etc
}

impl<'info> Make<'info> {
    pub fn init_escrow_state(&mut self, seeds: u8, bumps: MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            maker: self.maker.key(),
            seed: seeds,
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount: 0,
            bump: bumps.escrow,
        });
        Ok(())
    }

    // so depositing tokens we would send some amount from the user ata to the escrow ata
    // and we will be using cpi
    pub fn deposit_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_mint_a_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;
        Ok(())
    }
}