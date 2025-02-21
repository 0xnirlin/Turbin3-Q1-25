use anchor_lang::prelude::*;
use crate::instructions::{
    InitConfig,
    CreateOTCOrder,
    TakeOTCOrder,
};

pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("BMGeamZ9zF3nwgntjtoDJhKp4bn4U4CTJa1CZuyzVCNq");

#[program]
pub mod turbine_otc {
    use super::*;

    pub fn init_config(
        ctx: Context<InitConfig>,
        fee_percentage: u16,
        max_fee_percentage: u16,
        min_fee_percentage: u16,
        max_premium: u16,
        min_premium: u16,
        owner: Pubkey,
        listing_fee: u16,
    ) -> Result<()> {
        ctx.accounts.init(
            fee_percentage,
            max_fee_percentage,
            min_fee_percentage,
            max_premium,
            min_premium,
            ctx.bumps,
            owner,
            listing_fee,
        )
    }

    pub fn make_otc_order(
        ctx: Context<CreateOTCOrder>,
        amount: u64,
        seed: u64,
        seller: Option<Pubkey>,
        expiry_timestamp: u64,
        premium: u16,
    ) -> Result<()> {
        ctx.accounts.create_otc_order(amount, seed, ctx.bumps, seller, expiry_timestamp, premium)
    }

    pub fn cancel_otc_order(ctx: Context<CreateOTCOrder>) -> Result<()> {
        ctx.accounts.cancel_otc_order()
    }

    pub fn take_otc_order(ctx: Context<TakeOTCOrder>, amount_sol: u64) -> Result<()> {
        ctx.accounts.take_otc_order(amount_sol)
    }
}
