use anchor_lang::prelude::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::errors::VaultError;

use crate::states::CollateralVault;

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub new_balance: u64,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault", user.key().as_ref()],
        bump = vault.bump,
        constraint = vault.owner == user.key()
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == mint.key(),
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault_token_account.key() == vault.token_account,
        constraint = vault_token_account.mint == mint.key(),
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let vault = &mut self.vault;

        token_interface::transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.user_token_account.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault_token_account.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            amount,
            self.mint.decimals,
        )?;

        vault.total_balance = vault
            .total_balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        vault.available_balance = vault
            .available_balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        vault.total_deposited = vault
            .total_deposited
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;

        msg!("Deposited {} tokens into vault", amount);

        emit!(DepositEvent {
            user: self.user.key(),
            amount,
            new_balance: vault.total_balance,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
