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

    pub fn create_market(
        ctx: Context<CreateMarket>
    ) -> Result<()> {
        handler_create_market(
            ctx,
            1000, // base_lot_size: 1000, // This is a safe number for most markets
            1, // quote_lot_size
            1, // quote_dust_threshold
        )
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        nonce: u8,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64
    ) -> Result<()> {
        handler_add_liquidity(
            ctx,
            nonce,
            open_time,
            init_pc_amount,
            init_coin_amount
        )
    }
}