use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use instructions::*;

declare_id!("8K84Aq1aLXPTeV2ZfStqzzwU9G9GAdtcb8GR2C2LGvbC");

#[program]
pub mod collateral_vault {

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_bump: u8) -> Result<()> {
        ctx.accounts.initialize_vault(vault_bump)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn lock_collateral(ctx: Context<LockCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.lock(amount)
    }

    pub fn unlock_collateral(ctx: Context<UnlockCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.unlock(amount)
    }
}
