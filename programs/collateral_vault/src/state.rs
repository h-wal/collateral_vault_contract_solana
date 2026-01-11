use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CollateralVault {
    pub owner: Pubkey, //32
    pub token_account: Pubkey, //32

    pub total_balance: u64, //8
    pub locked_balance: u64, //8
    pub available_balance: u64, // 8

    pub total_deposited: u64, //8
    pub total_withdrawn: u64, //8

    pub created_at: i64, //8
    pub bump: u8, //1
    pub mint: Pubkey, //32
}

impl CollateralVault {
    pub const INIT_SPACE: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 32;
}
