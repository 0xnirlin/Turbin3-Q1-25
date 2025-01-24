use anchor_lang::prelude::*;
use crate::instructions::*;
use crate::state::*;

declare_id!("8CudMX4h9zyVMqPzjrqG7MoYWywNjG5qtwkG4FJpycw7");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u8, escrow_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow_state(seed, ctx.bumps)?;
        ctx.accounts.deposit_tokens(escrow_amount)?;
        Ok(())
    }

}

mod instructions;
mod state;
