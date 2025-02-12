use anchor_lang::prelude::*;

declare_id!("BMGeamZ9zF3nwgntjtoDJhKp4bn4U4CTJa1CZuyzVCNq");

#[program]
pub mod turbine_otc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
