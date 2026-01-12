use crate::errors::VaultError;
use crate::states::{CollateralVault, VaultAuthority};
use anchor_lang::prelude::*;

#[event]
pub struct CollateralTransferred {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct TransferCollateral<'info> {
    ///CHECK: validated via vault authority allowlist
    pub caller_program: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", from_vault.owner.as_ref()],
        bump = from_vault.bump
    )]
    pub from_vault: Account<'info, CollateralVault>,

    #[account(
        mut,
        seeds = [b"vault", to_vault.owner.as_ref()],
        bump = to_vault.bump
    )]
    pub to_vault: Account<'info, CollateralVault>,

    #[account(
        seeds = [b"vault_authority"],
        bump = vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultAuthority>,
}

impl<'info> TransferCollateral<'info> {
    pub fn transfer(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        require!(
            self.vault_authority
                .authorized_programs
                .contains(self.caller_program.key),
            VaultError::UnauthorizedCaller
        );

        require!(
            self.from_vault.mint == self.to_vault.mint,
            VaultError::MintMismatch
        );

        require!(
            self.from_vault.available_balance >= amount,
            VaultError::InsufficientAvailableBalance
        );

        require!(
            self.from_vault.key() != self.to_vault.key(),
            VaultError::InvalidTransfer
        );

        self.from_vault.available_balance = self
            .from_vault
            .available_balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.from_vault.total_balance = self
            .from_vault
            .total_balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.to_vault.available_balance = self
            .to_vault
            .available_balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.to_vault.total_balance = self
            .to_vault
            .total_balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        emit!(CollateralTransferred {
            from: self.from_vault.key(),
            to: self.to_vault.key(),
            amount,
        });

        Ok(())
    }
}
