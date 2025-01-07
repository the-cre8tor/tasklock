use anchor_lang::prelude::*;

mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("BzsbqDh3B1zyXVDWQWBTBYLgN5LDRxShxLmAutUZtMfT");

#[program]
pub mod tasklock {
    use super::*;

    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::InitializeProject::init(ctx, name, description)
    }
}
