use anchor_lang::prelude::*;
use events::*;

pub mod events;

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
