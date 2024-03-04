pub mod state;
pub mod utils;
pub mod instructions;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("Hd7gz8uBJy8LYQMhDFrKpemVVJtNYzRMi4qg8fWbrezD");

#[program]
pub mod s {
    use super::*;

    pub fn init_pool(
        ctx: Context<Initialize>,
        pool_name: String
    ) -> Result<()> {
        handler_init_pool(ctx, pool_name)
    }

    pub fn create_profile(
        ctx: Context<CreateProfile>
    ) -> Result<()> {
        handler_create_profile(ctx)
    }

    pub fn add(
        ctx: Context<Add>
    ) -> Result<()> {
        handler_add(
            ctx,
            1000, // base_lot_size: 1000, // This is a safe number for most markets
            1, // quote_lot_size
            1, // quote_dust_threshold
        )
    }
/*
    pub fn deposit(
        ctx: Context<Deposit>,
        sol_amount: u64
    ) -> Result<()> {
        handler_deposit(ctx, sol_amount)
    }

    pub fn claim(
        ctx: Context<Claim>,
    ) -> Result<()> {
        handler_claim(ctx)
    } */
}