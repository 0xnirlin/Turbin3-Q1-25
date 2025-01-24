use anchor_lang::prelude::*;
use crate::instructions::*;
use crate::state::*;

declare_id!("8CudMX4h9zyVMqPzjrqG7MoYWywNjG5qtwkG4FJpycw7");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, escrow_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow_state(seed, escrow_amount, &ctx.bumps)?;
        ctx.accounts.deposit(escrow_amount)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}

mod instructions;
mod state;

