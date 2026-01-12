use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid vault token account")]
    InvalidTokenAccount,

    #[msg("Invalid Amount Entered")]
    InvalidAmount,

    #[msg("Arithmetic overflow")]
    MathOverflow,

    #[msg("Unauthorized Caller")]
    UnauthorizedCaller,

    #[msg("Insufficient Available Balance")]
    InsufficientAvailableBalance
}
