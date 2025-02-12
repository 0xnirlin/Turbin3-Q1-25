use anchor_lang::prelude::*;

use crate::state::Listing;

// for creating the listings, the connfig must be initialized first. 
#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        space = Listing::SIZE,
        seeds = [b"turbine mint", token_mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"turbine_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, TurbineConfig>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

// there won't be option to close the listig, once it is open it is open yayy -  listing is created now anyone should be able to make an order in the listing.
impl<'info> CreateListing<'info> {
    pub fn create_listing(&mut self, listing_fee: u16, bumps: &CreateListingBumps) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.creator.to_account_info(),
            to: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, listing_fee)?;

        self.listing.set_inner(Listing {
            initiator: self.creator.key(),
            mint: self.token_mint.key(),
            bump: self.listing.bump,
        });


        Ok(())
    }
}

