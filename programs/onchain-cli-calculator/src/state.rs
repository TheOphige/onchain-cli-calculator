use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Calculator {
    pub calculator_authority: Pubkey,
    pub greeting: String,
    pub result: i64,
}
