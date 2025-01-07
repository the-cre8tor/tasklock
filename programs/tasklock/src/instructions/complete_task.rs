use anchor_lang::prelude::*;

use crate::errors::TaskError;
use crate::events::TaskCompleted;
use crate::state::{Project, Task, TaskStatus};

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

impl<'info> CompleteTask<'info> {
    pub fn run(ctx: Context<CompleteTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let project = &mut ctx.accounts.project;

        require!(
            task.status != TaskStatus::Completed,
            TaskError::TaskAlreadyCompleted
        );

        // Only assignee can complete task
        require!(
            ctx.accounts.assignee.key() == task.assignee,
            TaskError::UnauthorizedCompletion
        );

        task.status = TaskStatus::Completed;
        task.completed_at = Some(Clock::get()?.unix_timestamp);

        // Update project statistics safely
        project.active_tasks = project
            .active_tasks
            .checked_sub(1)
            .ok_or(TaskError::Overflow)?;

        project.completed_tasks = project
            .completed_tasks
            .checked_add(1)
            .ok_or(TaskError::Overflow)?;

        emit!(TaskCompleted {
            project: project.key(),
            task: task.key(),
            completed_by: ctx.accounts.assignee.key()
        });

        Ok(())
    }
}
