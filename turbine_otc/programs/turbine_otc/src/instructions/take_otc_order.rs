use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{Mint, TokenInterface, TokenAccount, transfer_checked},
    associated_token::AssociatedToken,
};
use raydium_clmm_cpi::{
    program::RaydiumClmm,
    states::{OBSERVATION_SEED, ObservationState, PoolState, AmmConfig, POOL_SEED},
};
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::tick_math::get_sqrt_price_at_tick;




use crate::state::{OTCOrderMaker, TurbineConfig};

use solana_program::native_token::LAMPORTS_PER_SOL;

// Option 1: Use the pubkey macro from anchor_lang instead
pub const FUNDING_AMOUNT: u64 = LAMPORTS_PER_SOL;
pub const WSOL_ID: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const LOCK_CPMM_AUTHORITY: Pubkey = pubkey!("3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH");
pub const DEFAULT_DECIMALS: u8 = 6;
pub const DEFAULT_SUPPLY: u64 = 1_000_000_000_000_000;
// when taking the order we want to transfer the tokens from the seller to the buyer's ATA
// and transfer the SOL from the vault to the seller

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance - The requested amount exceeds the available balance")]
    InsufficientBalance,
    #[msg("Order has expired - The order's expiration timestamp has passed")]
    OrderExpired,
    #[msg("Invalid seller - This order can only be taken by the specified seller")]
    InvalidSeller,
    #[msg("Invalid token amount - The calculated token amount is invalid or exceeds maximum value")]
    InvalidTokenAmount,
    #[msg("Math overflow occurred during calculation")]
    MathOverflow,
    #[msg("Tick upper overflow")]
    TickUpperOverflow,
}

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
    
    #[account(
        constraint = token_mint.key() == otc_order.token_mint
    )]
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
    pub config: Account<'info, TurbineConfig>,

    // Raydium CLMM accounts for TWAP price observation
    #[account(
        mut,
        seeds = [OBSERVATION_SEED.as_bytes(), pool_state.key().as_ref()],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    pub cp_swap_program: Program<'info, RaydiumClmm>,

    #[account(
        mut,
        address = WSOL_ID,
        constraint = base_mint.key() < token_mint.key(),
        mint::token_program = token_program,
    )]
    pub base_mint: Box<InterfaceAccount<'info, Mint>>,

    // The Raydium pool state for price reference
    #[account(
        mut,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            base_mint.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        seeds::program = cp_swap_program.key(),
        bump,
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    pub amm_config: Box<Account<'info, AmmConfig>>,

    pub raydium_clmm_program: Program<'info, RaydiumClmm>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TakeOTCOrder<'info> {
    fn transfer_sol(&self, amount_sol: u64, to: AccountInfo<'info>) -> Result<()> {
        let token_mint_key = self.token_mint.key();
        let vault_seeds = &[
            b"vault".as_ref(),
            token_mint_key.as_ref(),
            &[self.otc_order.vault_bump],
        ];
        let vault_signer = &[&vault_seeds[..]];

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to,
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, vault_signer);
        transfer(cpi_ctx, amount_sol)?;

        Ok(())
    }


    // let's say 1 sol is worth 1 token
    // When taking the order, the seller provides tokens and receives SOL with a premium discount
    pub fn take_otc_order(&mut self, amount_sol: u64) -> Result<()> {
        // Check if the requested amount is valid
        if amount_sol > self.otc_order.sol_amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        // Check if order has expired
        if Clock::get()?.unix_timestamp > self.otc_order.expiration_timestamp as i64 {
            return Err(ErrorCode::OrderExpired.into());
        }

        // Verify seller if specified
        if let Some(specified_seller) = self.otc_order.seller {
            if self.seller.key() != specified_seller {
                return Err(ErrorCode::InvalidSeller.into());
            }
        }

        // Calculate TWAP using Raydium CLMM
        const OBSERVATION_NUM: usize = 100;
        let latest_index = self.observation_state.load()?.observations.len() - 1;
        
        let mut sum_sqrt_price = 0u128;
        
        // Average last 10 observations
        for i in 0..10 {
            let index = (latest_index + OBSERVATION_NUM - i) % OBSERVATION_NUM;
            let sqrt_price = get_sqrt_price_at_tick(
                self.observation_state.load()?.observations[index].sqrt_price_x64 as i32
            )?;
            sum_sqrt_price = sum_sqrt_price.checked_add(sqrt_price as u128)
                .ok_or(ErrorCode::MathOverflow)?;
        }

        let twap_sqrt_price_x64 = (sum_sqrt_price / 10) as u64;

        // Calculate price from sqrt price
        let twap_price_x64 = (twap_sqrt_price_x64 as u128)
            .checked_mul(twap_sqrt_price_x64 as u128)
            .ok_or(ErrorCode::MathOverflow)?
            >> 64;

        // Calculate token amount based on pool token ordering
        let token_amount = if self.pool_state.load()?.token_mint_0 == self.token_mint.key() {
            // token0/SOL price
            (amount_sol as u128)
                .checked_mul(twap_price_x64)
                .ok_or(ErrorCode::MathOverflow)?
                >> 64
        } else {
            // SOL/token1 price
            ((amount_sol as u128) << 64)
                .checked_div(twap_price_x64)
                .ok_or(ErrorCode::MathOverflow)?
        };

        // Apply premium discount
        let token_amount_with_premium = token_amount
            .checked_sub((token_amount * self.otc_order.premium as u128) / 100)
            .ok_or(ErrorCode::MathOverflow)?;

        if token_amount_with_premium > u64::MAX as u128 {
            return Err(ErrorCode::InvalidTokenAmount.into());
        }

        // Transfer SOL from vault to seller
        self.transfer_sol(amount_sol, self.seller.to_account_info())?;

        // Transfer tokens from seller to buyer
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token_interface::TransferChecked {
            from: self.seller_token_account.to_account_info(),
            mint: self.token_mint.to_account_info(),
            to: self.buyer_token_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, token_amount_with_premium as u64, self.token_mint.decimals)?;

        // Update remaining order amount
        self.otc_order.sol_amount = self.otc_order.sol_amount
            .checked_sub(amount_sol)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }
}
