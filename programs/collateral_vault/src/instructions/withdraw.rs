use anchor_lang::prelude::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::errors::VaultError;
use crate::states::CollateralVault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault" , vault.owner.as_ref()],
        bump = vault.bump,
        constraint = vault.owner == user.key()
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(
        mut,
        constraint = vault_token_account.key() == vault.token_account,
        constraint = vault_token_account.mint == mint_account.key()
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == mint_account.key()
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        require!(
            self.vault.available_balance >= amount,
            VaultError::InsufficientAvailableBalance
        );

        //pda signer
        let signer_seeds: &[&[u8]] = &[b"vault", self.vault.owner.as_ref(), &[self.vault.bump]];

        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault_token_account.to_account_info(),
                    mint: self.mint_account.to_account_info(),
                    to: self.user_token_account.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
            self.mint_account.decimals,
        )?;

        self.vault.total_balance = self
            .vault
            .total_balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.vault.available_balance = self
            .vault
            .available_balance
            .checked_sub(amount)
            .ok_or(VaultError::MathOverflow)?;

        self.vault.total_withdrawn = self
            .vault
            .total_withdrawn
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        Ok(())
    }
}
