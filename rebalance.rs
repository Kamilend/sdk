use anchor_lang::prelude::*;
use crate::state::Vault;

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

pub fn handler(_ctx: Context<Rebalance>) -> Result<()> {
    // Hook to external protocols:
    // - Kamino
    // - Solend
    // - Marginfi
    Ok(())
}
