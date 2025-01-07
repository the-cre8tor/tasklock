use anchor_lang::prelude::*;

use crate::errors::TaskError;
use crate::events::ProjectCreated;
use crate::state::Project;

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct InitializeProject<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Project::space(&name, &description),
        seeds = [b"project", owner.key().as_ref()],
        bump
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProject<'info> {
    const MAX_NAME_LEN: usize = 50;
    const MAX_DESCRIPTION_LEN: usize = 200;

    pub fn init(ctx: Context<InitializeProject>, name: String, description: String) -> Result<()> {
        require!(name.len() <= Self::MAX_NAME_LEN, TaskError::NameTooLong);
        require!(
            description.len() <= Self::MAX_DESCRIPTION_LEN,
            TaskError::DescriptionTooLong
        );

        let project = &mut ctx.accounts.project;

        project.owner = ctx.accounts.owner.key();
        project.name = name;
        project.description = description;
        project.task_count = 0;
        project.active_tasks = 0;
        project.completed_tasks = 0;
        project.created_at = Clock::get()?.unix_timestamp;
        project.bump = ctx.bumps.project;

        emit!(ProjectCreated {
            project: project.key(),
            owner: project.owner,
            name: project.name.clone()
        });

        Ok(())
    }
}
