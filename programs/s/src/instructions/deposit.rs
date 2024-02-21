use anchor_lang::prelude::*;
use crate::{state::{Pool, Profile}, utils::transfer_sol};

pub fn handler_deposit(ctx: Context<Deposit>, sol_amount: u64) -> Result<()> {
    transfer_sol(
        ctx.accounts.user.to_account_info(),
        ctx.accounts.program_native_account.to_account_info(),
        sol_amount,
        Option::None,
        ctx.accounts.system_program.to_account_info()
    )?;

    let profile = &mut ctx.accounts.profile;
    profile.point = sol_amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(xsb_amount: u64)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [pool.name.as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    /// CHECK
    #[account(
        mut,
        seeds = [pool.key().as_ref()],
        bump
    )]
    pub program_native_account: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        has_one=user.key(),
        has_one=pool.key()
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}