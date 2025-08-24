use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::InitializeCalculatorEvent;

#[derive(Accounts)]
pub struct InitializeCalculator<'info> {
    #[account(mut)]
    pub calculator_authority: Signer<'info>,
    #[account(
        init, 
        payer = calculator_authority, 
        // space = discriminant + account size
        space = 8 + Calculator::INIT_SPACE,
        seeds = [b"calculator", calculator_authority.key().as_ref()],
        bump
    )]
    pub calculator: Account<'info, Calculator>,
    pub system_program: Program<'info, System>,
}

pub fn _init_calculator(ctx: Context<InitializeCalculator>, greeting: String) -> Result<()> {
    let calculator = &mut ctx.accounts.calculator;

    calculator.greeting = greeting;
    calculator.result = 0;

    emit!(InitializeCalculatorEvent {
        calculator: calculator.key(),
        calculator_authority: ctx.accounts.calculator_authority.key(),
        locked: true
    });

  Ok(())
}