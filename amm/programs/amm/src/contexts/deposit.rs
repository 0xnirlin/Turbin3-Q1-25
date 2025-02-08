use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        Mint, TokenAccount, TokenInterface,
        transfer_checked, mint_to, TransferChecked, MintTo
    },
    associated_token::AssociatedToken
};

use crate::error::*;
use crate::state::Config;
use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    // we need the pool config to get the bumps for the lp and get the tokens mint. 
    #[account(
        has_one = token_a_mint,
        has_one = token_b_mint,
        seeds = [b"config", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), config.seed.as_bytes()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    // we need both tokens ata and lp ata
    // both tokens ata to transfer in the tokens to the pool
    // and mint ata because we will mint lp tokens to the signer
    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = signer,
    )]
    pub token_a_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = signer,
    )]
    pub token_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = lp_mint,
        associated_token::authority = signer,
    )]
    pub lp_token_ata: InterfaceAccount<'info, TokenAccount>,

    // we need the right seeds for this
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub lp_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = config,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = config,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, lp_amount: u64, max_a: u64, max_b: u64) -> Result<()> {
        require!(lp_amount > 0, AmlError::InvalidLpAmount);
        require!(!self.config.locked, AmlError::PoolLocked);

        // now if in the config
        let (x,y) = match self.lp_mint.supply == 0 && self.vault_a.amount == 0 && self.vault_b.amount == 0 {
            true => (max_a, max_b),
            false => {
                //other wise calculate

                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_a.amount,
                    self.vault_b.amount,
                    self.lp_mint.supply,
                    lp_amount,
                    6,
                ).unwrap();

                (amounts.x, amounts.y)
            }
        };

        require!(x <= max_a, AmlError::MaxA);
        require!(y <= max_b, AmlError::MaxB);

        // now we will send x and y tokens separaptely from the user into their vaults
        // first arg is amount of tokens and second arg is to do x or y, if true x and if false y
        self.deposit_token(x, true);
        self.deposit_token(y, false);
        self.mint_lp_tokens(lp_amount);
        
        
        Ok(())
    }

    fn deposit_token(&self, amount: u64, is_x: bool) -> Result<()> {
        // first we get the cpi accounts
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (
                TransferChecked {
                    from: self.signer.to_account_info(),
                    to: self.vault_a.to_account_info(),
                    mint: self.token_a_mint.to_account_info(),
                    authority: self.signer.to_account_info(),
                }, 
                self.token_a_mint.decimals
            ),
            
            false => (
                TransferChecked {
                    from: self.signer.to_account_info(),
                    to: self.vault_b.to_account_info(),
                    mint: self.token_b_mint.to_account_info(),
                    authority: self.signer.to_account_info(),
                }, 
                self.token_b_mint.decimals
            )
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, mint_decimals)?;

        Ok(())
    }

    fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        // first we get the cpi accounts
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.lp_mint.to_account_info(),
            to: self.lp_token_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let token_a_key = self.token_a_mint.to_account_info().key();
        let token_b_key = self.token_b_mint.to_account_info().key();
        let seeds = &[b"lp", token_a_key.as_ref(), token_b_key.as_ref(), self.config.seed.as_bytes(), &[self.config.config_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount);

        Ok(())
    }

}
