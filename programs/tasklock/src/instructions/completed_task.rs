use anchor_lang::prelude::*;

use crate::state::{Project, Task};

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub assignee: Signer<'info>,

    #[account(
        mut,
        seeds = [b"project", project.owner.as_ref()],
        bump = project.bump
    )]
    pub project: Account<'info, Project>,

    #[account(
        mut,
        seeds = [
            b"task",
            project.key().as_ref(),
            task.task_id.to_le_bytes().as_ref()
        ],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,
}
