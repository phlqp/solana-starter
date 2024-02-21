use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{state::{Pool, Profile}, utils::transfer_token};

pub fn handler_claim(ctx: Context<Claim>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let profile = &mut ctx.accounts.profile;

    let seeds = &[pool.name.as_ref(), &[pool.bump]];

    transfer_token(
        ctx.accounts.program_token_account.to_account_info(),
        ctx.accounts.user_token_account.to_account_info(),
        pool.to_account_info(),
        profile.point,
        Some(&[&seeds[..]]),
        ctx.accounts.token_program.to_account_info()
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(
        mut,
        seeds = [],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = pool.key(),
    )]
    pub program_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        has_one=user.key(),
        has_one=pool.key()
    )]
    pub profile: Account<'info, Profile>,
    #[account(
        mut,
        token::mint = pool.mint,
        token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}