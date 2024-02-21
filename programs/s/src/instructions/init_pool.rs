use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken
};

use crate::state::Pool;

pub fn handler_init_pool(ctx: Context<Initialize>, pool_name: String) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let name_bytes = pool_name.as_bytes();
    let mut name_data = [b' '; 10];
    name_data[..name_bytes.len()].copy_from_slice(name_bytes);

    pool.name = name_data;
    pool.authority = *ctx.accounts.authority.key;
    pool.mint = ctx.accounts.mint.key();
    pool.bump = ctx.bumps.pool;

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [pool_name.as_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    pub mint: Account<'info, Mint>, // token mint address
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = pool,
    )]
    pub program_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}