use anchor_lang::prelude::*;
use crate::state::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"vault", asset_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub asset_mint: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.asset_mint = ctx.accounts.asset_mint.key();
    vault.total_deposits = 0;
    vault.bump = *ctx.bumps.get("vault").unwrap();
    Ok(())
}
