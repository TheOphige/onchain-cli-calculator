use anchor_lang::prelude::*;

#[error_code]
pub enum CalculatorError {
    #[msg("Calculator is locked")]
    CalculatorLocked,
    #[msg("Overflow")]
    Overflow,
    #[msg("Insufficient balance")]
    InsufficientBalance,
}