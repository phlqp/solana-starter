use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::token::{Mint, TokenAccount};

pub fn handler_add_liquidity(
    ctx: Context<AddLiquidity>,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64
) -> Result<()> {
    let instruction = raydium_contract_instructions::amm_instruction::initialize2(
        ctx.accounts.raydium_program.key, // program id
        ctx.accounts.amm_id.key, // amm_id
        ctx.accounts.user.key, // amm_authority,
        ctx.accounts.amm_openorder.key, // amm_open_orders,
        &ctx.accounts.lp_mint.key(), // lp_mint_address,
        &ctx.accounts.coin_mint.key(), // coin_mint_address,
        &ctx.accounts.pc_mint.key(), // pc_mint_address,
        &ctx.accounts.pool_coin_token_account.key(), //  pool_coin_token_account,
        &ctx.accounts.pool_pc_token_account.key(), //  pool_pc_token_account,
        ctx.accounts.pool_withdraw_queue.key, //  pool_withdraw_queue,
        ctx.accounts.amm_target_orders.key, //  amm_target_orders,
        ctx.accounts.pool_temp_lp.key, // pool_temp_lp        ,
        ctx.accounts.openbook_program.key, // openbook_program_id,
        ctx.accounts.openbook_market.key, // openbook_market,
        ctx.accounts.user.key, // user
        &ctx.accounts.program_token_coin.key(), // program or user_token_coin,
        &ctx.accounts.program_token_pc.key(), // program or user_token_pc,
        &ctx.accounts.user_token_lp.key(), // Contain
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount
    )?;

    invoke_signed(
        &instruction,
        &ctx.accounts.to_account_infos(),
        &[]
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub raydium_program: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    pub amm_openorder: UncheckedAccount<'info>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub coin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub pc_mint: Account<'info, Mint>,
    #[account(mut)]
    pub pool_coin_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_pc_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_withdraw_queue: UncheckedAccount<'info>,
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub pool_temp_lp: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub program_token_coin: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_token_pc: Account<'info, TokenAccount>,
    pub user_token_lp: Account<'info, TokenAccount>,
    pub openbook_program: AccountInfo<'info>,
    pub openbook_market: AccountInfo<'info>,

}