use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, TokenAccount, transfer_checked, TransferChecked};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = depositor
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = escrow_account
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: Box<InterfaceAccount<'info, TokenInterface>>,

    /// CHECK: PDA owned by this program
    #[account(
        seeds = [b"escrow", depositor.key().as_ref()],
        bump
    )]
    pub escrow_account: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.depositor.key().as_ref(),
            &[*ctx.bumps.get("escrow_account").unwrap()],
        ]];

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.depositor_token_account.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                    to: self.escrow_token_account.to_account_info(),
                    authority: self.depositor.to_account_info(),
                },
            ),
            amount,
            self.token_mint.decimals,
        )
    }
} 