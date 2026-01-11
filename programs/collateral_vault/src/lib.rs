use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod constants;


pub use instructions::*;

declare_id!("8K84Aq1aLXPTeV2ZfStqzzwU9G9GAdtcb8GR2C2LGvbC");

#[program]
pub mod collateral_vault {

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_bump: u8) -> Result<()> {

        ctx.accounts.initialize_vault(vault_bump)
    }
}
