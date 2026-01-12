use anchor_lang::prelude::*;

use crate::errors::VaultError;
use crate::states::{CollateralVault, VaultAuthority};

#[derive(Accounts)]

pub struct LockCollateral<'info> {
    ///CHECK: validated against vault authority
    pub caller_program: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds= [b"vault", vault.owner.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(
        seeds = [b"vault_authority"],
        bump = vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultAuthority>,
}

impl<'info> LockCollateral<'info> {
    pub fn lock(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        require!(
            self.vault_authority
                .authorized_programs
                .contains(self.caller_program.key),
            VaultError::UnauthorizedCaller
        );

        require!(
            self.vault.available_balance >= amount,
            VaultError::InsufficientAvailableBalance
        );

        self.vault.available_balance = self
            .vault
            .available_balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.vault.locked_balance = self
            .vault
            .locked_balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        Ok(())
    }
}
