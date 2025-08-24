use anchor_lang::prelude::*;
use crate::errors::CalculatorError;
use crate::events::AdditionEvent;

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        constraint = !vault.locked @ VaultError::VaultLocked
    )]
    pub calculator: Account<'info, Calculator>,

    pub system_program: Program<'info, System>,
}

pub fn _add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;
    let user = &ctx.accounts.user;

    // Verify user has enough balance
    require!(
        user.to_account_info().lamports() >= amount,
        VaultError::InsufficientBalance
    );
    
    calculator.result = num1 + num2;
    msg!("{} + {} = {}", num1, num2, calculator.result);
        
    // Emit deposit event
    emit!(AdditionEvent {
        vault: vault.key(),
        user: user.key(),
        amount,
    });
        
    Ok(())
    }