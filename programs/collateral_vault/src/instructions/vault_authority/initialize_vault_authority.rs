use crate::states::VaultAuthority;
use anchor_lang::prelude::*;

#[event]
pub struct VaultAuthorityInitialized {
    pub admin: Pubkey,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeVaultAuthority<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"vault_authority"],
        bump,
        space = 8 + VaultAuthority::INIT_SPACE
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVaultAuthority<'info> {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.vault_authority.admin = self.payer.key();
        self.vault_authority.authorized_programs = vec![];
        self.vault_authority.bump = bump;

        emit!(VaultAuthorityInitialized {
            admin: self.payer.key(),
        });

        Ok(())
    }
}
