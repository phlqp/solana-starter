use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub name: [u8; 10], // Setting an arbitrary max of ten characters in the name. // 10
    pub bump: u8,
    pub authority: Pubkey, // Administrator's key
    pub mint: Pubkey, // token mint address
}

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub pool: Pubkey,
    pub user: Pubkey,
    pub point: u64
}
