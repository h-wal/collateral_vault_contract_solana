use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;

//use crate::constants::USDC_MINT;
use crate::state::CollateralVault;

#[derive(Accounts)]
#[instruction(vault_bump: u8)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // #[account(
    //     constraint= mint.key() == USDC_MINT
    // )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,
        space = 8 + CollateralVault::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, CollateralVault>, //create the vault account

    #[account(
        init,
        payer = user, 
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>, //token account will be validated later

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, vault_bump: u8) -> Result<()> {

        let vault = &mut self.vault;

        vault.owner = self.user.key();
        vault.token_account = self.vault_token_account.key();

        vault.total_balance = 0;
        vault.locked_balance = 0;
        vault.available_balance = 0;

        vault.total_deposited = 0;
        vault.total_withdrawn = 0;

        vault.created_at = Clock::get()?.unix_timestamp;
        vault.bump = vault_bump;
        vault.mint = self.mint.key();

        Ok(())
    }
}
