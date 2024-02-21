use anchor_lang::prelude::*;

use crate::state::{Pool, Profile};

pub fn handler_create_profile(ctx: Context<CreateProfile>) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.pool = ctx.accounts.pool.key();
    profile.user = *ctx.accounts.user.key;
    profile.point = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        mut,
        seeds = [pool.name.as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Profile::INIT_SPACE,
        seeds = [pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}