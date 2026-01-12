use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultAuthority {

    pub admin: Pubkey,
    #[max_len(10)]
    pub authorized_programs: Vec<Pubkey>,
    pub bump: u8,
}

impl VaultAuthority {
    pub const INIT_SPACE: usize = 32 + 4 + (32 * 10) + 1;
}
