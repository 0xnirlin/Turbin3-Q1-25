use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program::{Transfer, transfer};


declare_id!("EU6hmnrN9E2933DZA1ba2JTzEf9czKSqkEqcKhLA7mak");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", signer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
    pub token_deposit: u64,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.token_deposit = 0;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        // using cpi transfer lamports to the vault
        let system_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(system_program, accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
   pub fn withdraw(&self, amount: u64) -> Result<()> {
    let system_program = self.system_program.to_account_info();
    
    // Now use the binding in seeds
    let vault_seeds = &[
        self.vault_state.to_account_info().key.as_ref(),
        &[self.vault_state.vault_bump]
    ];

    let signer_seeds = &[&vault_seeds[..]];

    let accounts = Transfer {
        from: self.vault.to_account_info(),
        to: self.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        system_program, 
        accounts,
        signer_seeds
    );

    transfer(cpi_ctx, amount)?;
    Ok(())
}

}
