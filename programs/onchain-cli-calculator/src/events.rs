use anchor_lang::prelude::*;

#[event]
pub struct InitializeCalculatorEvent {
    pub calculator: Pubkey,
    pub calculator_authority: Pubkey,
    pub locked: bool,
}

#[event]
pub struct AdditionEvent {
    pub amount: u64,
    pub user: Pubkey,
    pub vault: Pubkey,
}

#[event]
pub struct WithdrawEvent {
    pub amount: u64,
    pub vault_authority: Pubkey,
    pub vault: Pubkey,
}

#[event]
pub struct ToggleLockEvent {
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub locked: bool,
}