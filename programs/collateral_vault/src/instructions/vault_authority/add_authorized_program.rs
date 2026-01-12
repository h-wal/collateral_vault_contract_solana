use crate::errors::VaultError;
use crate::states::VaultAuthority;
use anchor_lang::prelude::*;

#[event]
pub struct ProgramAuthorized {
    pub program_id: Pubkey,
}

#[derive(Accounts)]
pub struct AddAuthorizedPrograms<'info> {
    #[account(
        mut,
        seeds = [b"vault_authority"],
        bump = vault_authority.bump
    )]
    pub vault_authority: Account<'info, VaultAuthority>,

    pub admin: Signer<'info>,
}

impl<'info> AddAuthorizedPrograms<'info> {
    pub fn add(&mut self, program_id: Pubkey) -> Result<()> {
        require!(
            self.vault_authority.admin == self.admin.key(),
            VaultError::UnauthorizedAdmin
        );

        require!(
            !self
                .vault_authority
                .authorized_programs
                .contains(&program_id),
            VaultError::ProgramAlreadyAuthorized
        );

        self.vault_authority.authorized_programs.push(program_id);

        emit!(ProgramAuthorized { program_id });

        Ok(())
    }
}
