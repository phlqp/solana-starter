pub mod state;
pub mod utils;
pub mod instructions;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("4cfnzBx8GxZXCJ3P1o62YrMnBMfT5EcVBvGqgo7fFLkc");

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