use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub asset_mint: Pubkey,
    pub total_deposits: u64,
    pub bump: u8,
}
