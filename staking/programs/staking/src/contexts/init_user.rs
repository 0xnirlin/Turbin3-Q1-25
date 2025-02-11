use anchor_lang::prelude::*;
use crate::state::User;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
        payer = payer,
        space = User::INIT_SPACE,
        seeds = [b"user".as_ref(), payer.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self, bumps: &InitUserBumps) {
        // set total stake to 0
        // set last update to current timestamp 
        self.user.set_inner(User {
            bump: bumps.user,
            total_staked: 0,
            last_update: Clock::get().unwrap().unix_timestamp,
        });
    }
}