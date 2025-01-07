use crate::{Project, Task};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"project", owner.key().as_ref()],
        bump = project.bump,
        has_one = owner
    )]
    pub project: Account<'info, Project>,

    #[account(
        init,
        payer = owner,
        space = Task::space(&title, &description),
        seeds = [
            b"task",
            project.key().as_ref(),
            project.task_count.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    pub system_program: Program<'info, System>,
}
