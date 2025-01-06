mod errors;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;

use errors::*;
use events::*;
use instruction::*;
use state::*;

declare_id!("BzsbqDh3B1zyXVDWQWBTBYLgN5LDRxShxLmAutUZtMfT");

#[program]
pub mod tasklock {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
