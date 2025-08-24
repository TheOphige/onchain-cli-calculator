use anchor_lang::prelude::*;
mod instructions;
mod state;
mod errors;
mod events;

use instructions::*;

declare_id!("EczkwmE3dX51byJGRJbY7gJ7LK2x74QhRuBZgcEWwFzq");

#[program]
pub mod onchain_cli_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        _init_calculator(ctx, init_message)
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
        _add(ctx, num1, num2)
    }
}