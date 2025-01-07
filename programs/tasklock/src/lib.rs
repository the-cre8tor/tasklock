use anchor_lang::prelude::*;

mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("BzsbqDh3B1zyXVDWQWBTBYLgN5LDRxShxLmAutUZtMfT");

pub const MAX_NAME_LEN: usize = 50;
pub const MAX_DESCRIPTION_LEN: usize = 200;

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
