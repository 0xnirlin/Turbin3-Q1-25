use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
};
use crate::state::escrow::Escrow;

#[derive(Accounts)]

pub struct Take<'info> {
    // taker is the signer
    #[account(mut)]
    pub taker: Signer<'info>,



    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    // we the the maker ata
    #[account(init_if_needed, payer = taker, associated_token::mint = mint_a, associated_token::authority = taker)]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed, payer = taker, associated_token::mint = mint_b, associated_token::authority = escrow.maker)]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut, associated_token::mint = mint_a, associated_token::authority = escrow)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}