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
    InsufficientAvailableBalance,

    #[msg("Insufficient Locked Balance")]
    InsufficientLockedBalance,

    #[msg("Program Already Authorized")]
    ProgramAlreadyAuthorized,

    #[msg("Unauthorized Admin")]
    UnauthorizedAdmin,

    #[msg("Mint Mismatch")]
    MintMismatch,

    #[msg("Invalid Transfer")]
    InvalidTransfer,
}
