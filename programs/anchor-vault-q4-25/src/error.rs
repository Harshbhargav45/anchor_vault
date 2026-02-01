use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The requested quantity is invalid.")]
    InvalidAmount,
    #[msg("The vault is empty.")]
    VaultEmpty,
    #[msg("Withdrawal amount exceeds vault balance.")]
    InsufficientFunds,
}
