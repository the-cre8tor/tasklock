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

    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
        deadline: i64,
    ) -> Result<()> {
        instructions::CreateTask::run(ctx, title, description, deadline)
    }

    pub fn assign_task(ctx: Context<AssignTask>, new_assignee: Pubkey) -> Result<()> {
        instructions::AssignTask::run(ctx, new_assignee)
    }
}
