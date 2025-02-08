use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, mint_to}, associated_token::AssociatedToken};

declare_id!("EHpjvfAcZAW9TEdRzhw7KD34g8Htwoqc1np4aUK7JsxP");

pub mod contexts;
pub mod state;
pub mod error;

use crate::contexts::*;
use crate::state::*;

#[program]
pub mod amm {
    use super::*;

    pub fn init(ctx: Context<Init>, seed: String) -> Result<()> {
        ctx.accounts.init(500, None, seed, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, lp_amount: u64, max_a: u64, max_b: u64) -> Result<()> {
        ctx.accounts.deposit(lp_amount, max_a, max_b)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
