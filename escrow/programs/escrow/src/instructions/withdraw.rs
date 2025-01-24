use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, TokenAccount, transfer_checked, TransferChecked};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = owner
    )]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = escrow_account
    )]
    pub escrow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: Box<InterfaceAccount<'info, TokenInterface>>,

    /// CHECK: PDA owned by this program
    #[account(
        seeds = [b"escrow", owner.key().as_ref()],
        bump
    )]
    pub escrow_account: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.owner.key().as_ref(),
            &[*ctx.bumps.get("escrow_account").unwrap()],
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.escrow_token_account.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                    to: self.owner_token_account.to_account_info(),
                    authority: self.escrow_account.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            self.token_mint.decimals,
        )
    }
} 