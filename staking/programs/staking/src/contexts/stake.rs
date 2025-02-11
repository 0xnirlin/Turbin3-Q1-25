use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount
    },
    token::{Token, Approve, approve},
    token_interface::{Mint, TokenAccount},
};

use crate::state::{Stake, Config, User};
use crate::StakingError;


#[derive(Accounts)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Account<'info, Mint>,


    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    // we need to create the stake account
    #[account(
        init,
        payer = payer,
        space = Stake::INIT_SPACE,
        seeds = [b"stake", mint.key().as_ref(), payer.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, Stake>,
    #[account(
        seeds = [b"metadata",metadata_program.key().as_ref(), mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key_as_ref() == config.collection_key.as_ref(), 
        constraint = metadata.collection.as_ref().unwrap().verified == true,
        )]
    pub metadata: Account<'info, MetadataAccount>,

    pub metadata_program: Program<'info, Metadata>,
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref(),b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [b"user", payer.key().as_ref()],
        bump = user.bump,
    )]
    pub user: Account<'info, User>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

impl<'info> StakeNFT<'info> {
    pub fn stake(&mut self, bumps: &StakeNFTBumps) {
        self.stake_account.set_inner(Stake {
            mint: self.mint.key(),
            last_update: Clock::get().unwrap().unix_timestamp,
            bump: bumps.stake_account,
        });

        // now we want to approve the stake_account for the nft
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            from: self.user_ata.to_account_info(),
            to: self.stake_account.to_account_info(),
            authority: self.payer.to_account_info(),
        };

        let cpi_ctx = Context::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        self.user.total_staked += 1;
        self.user.last_update = Clock::get().unwrap().unix_timestamp;

        // stake account seeds
        let seeds = &[b"stake", self.mint.key().as_ref(), self.payer.key().as_ref(), &[bumps.stake_account]];
        let signer_seeds = &[&seeds[..]];
       
        FreezeDelegatedAccountCpi::new(
            self.metadata_program.to_account_info(),
            FreezeDelegatedAccountCpiAccounts {
                delegated_account: self.stake_account.to_account_info(),
                delegated_authority: self.user_ata.to_account_info(),
                edition: self.edition.to_account_info(),
                mint: self.mint.to_account_info(),
                token_program: self.token_program.to_account_info(),
            },
           
        ).invoke_signed(signer_seeds)?;

        // require stake amount is within limits
        require!(
            self.user.total_staked <= self.config.max_stake,
            StakingError::StakeAmountTooHigh
        );
    }
}