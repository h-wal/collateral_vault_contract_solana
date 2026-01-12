use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultAuthority {
    #[max_len(10)]
    pub authorized_programs: Vec<Pubkey>,
    pub bump: u8,
}

impl VaultAuthority {
    pub const INIT_SPACE: usize = 4 + (32 * 10) + 1;
}
