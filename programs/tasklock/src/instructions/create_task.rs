use anchor_lang::prelude::*;

use crate::errors::TaskError;
use crate::events::TaskCreated;
use crate::state::{Project, Task, TaskStatus};

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

impl<'info> CreateTask<'info> {
    pub fn run(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
        deadline: i64,
    ) -> Result<()> {
        require!(title.len() <= 50, TaskError::TitleTooLong);
        require!(description.len() <= 200, TaskError::TitleTooLong);
        require!(!title.trim().is_empty(), TaskError::EmptyTitle);
        require!(!description.trim().is_empty(), TaskError::EmptyDescription);

        let current_time = Clock::get()?.unix_timestamp;
        require!(deadline > current_time, TaskError::InvalidDeadline);

        let project = &mut ctx.accounts.project;
        let task = &mut ctx.accounts.task;

        require!(project.task_count < u32::MAX, TaskError::MaxTasksReached);

        task.project = project.key();
        task.title = title;
        task.description = description;
        task.creator = ctx.accounts.owner.key();
        task.assignee = ctx.accounts.owner.key();
        task.status = TaskStatus::Created;
        task.created_at = current_time;
        task.deadline = deadline;
        task.completed_at = None;
        task.task_id = project.task_count;
        task.bump = ctx.bumps.task;

        project.task_count = project
            .task_count
            .checked_add(1)
            .ok_or(TaskError::Overflow)?;

        project.active_tasks = project
            .active_tasks
            .checked_add(1)
            .ok_or(TaskError::Overflow)?;

        emit!(TaskCreated {
            project: project.key(),
            task: task.key(),
            creator: task.creator,
            title: task.title.clone()
        });

        Ok(())
    }
}
