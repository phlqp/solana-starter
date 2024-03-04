use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::{id, state::{Pool, Profile}, utils::transfer_sol};

pub fn handler_add(
    ctx: Context<Add>,
    coin_lot_size: u64,
    pc_lot_size: u64,
    pc_dust_threshold: u64
) -> Result<()> {
    let (vault_signer_nonce, _vault_signer) = {
        let mut i = 0;
        loop {
            assert!(i < 100);
            if let Ok(pk) =
                anchor_spl::dex::serum_dex::state::gen_vault_signer_key(i, &ctx.accounts.market_state.key(), &id())
            {
                break (i, pk);
            }
            i += 1;
        }
    };

    let context = CpiContext::new_with_signer(
        ctx.accounts.dex_program.to_account_info(),
        anchor_spl::dex::InitializeMarket {
            market: ctx.accounts.market_state.to_account_info(),
            coin_mint: ctx.accounts.mint_base.to_account_info(),
            pc_mint: ctx.accounts.mint_quote.to_account_info(),
            coin_vault: ctx.accounts.vault_base.to_account_info(),
            pc_vault: ctx.accounts.vault_quote.to_account_info(),
            bids: ctx.accounts.bids.to_account_info(),
            asks: ctx.accounts.asks.to_account_info(),
            req_q: ctx.accounts.request_queue.to_account_info(),
            event_q: ctx.accounts.event_queue.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        &[],
    );

    // Create OpenBook market id
    anchor_spl::dex::initialize_market(
        context,
        coin_lot_size,
        pc_lot_size,
        vault_signer_nonce,
        pc_dust_threshold
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    mint_base: Box<Account<'info, Mint>>,
    #[account(mut)]
    mint_quote: Box<Account<'info, Mint>>,
    #[account(init,
        seeds = [
            b"openbook-market",
          mint_base.key().as_ref(),
          mint_quote.key().as_ref(),
        ],
        bump,
        space = 12 + std::mem::size_of::<anchor_spl::dex::serum_dex::state::MarketState>(),
        owner = anchor_spl::dex::ID,
        payer = user
    )]
    pub market_state: AccountInfo<'info>,
    vault_signer: AccountInfo<'info>,

    #[account(init,
              seeds = [
                b"swap-pool-tokens",
                market_state.key().as_ref(),
                mint_base.key().as_ref()
              ],
              bump,
              token::mint = mint_base,
              token::authority = vault_signer,
              payer = user,
    )]
    vault_base: Box<Account<'info, TokenAccount>>,
    #[account(init,
              seeds = [
                b"swap-pool-tokens",
                market_state.key().as_ref(),
                mint_quote.key().as_ref()
              ],
              bump,
              token::mint = mint_quote,
              token::authority = vault_signer,
              payer = user,
    )]
    vault_quote: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    bids: AccountInfo<'info>,
    #[account(mut)]
    asks: AccountInfo<'info>,
    #[account(mut)]
    request_queue: AccountInfo<'info>,
    #[account(mut)]
    event_queue: AccountInfo<'info>,

    pub dex_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}