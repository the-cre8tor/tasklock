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
    /// Initializes a new project with the given name and description
    /// Parameters:
    ///   - name: The name of the project (max 50 chars)
    ///   - description: The project description (max 200 chars)
    pub fn init(ctx: Context<InitializeProject>, name: String, description: String) -> Result<()> {
        require!(name.len() <= 20, TaskError::NameTooLong);
        require!(description.len() <= 200, TaskError::DescriptionTooLong);
        require!(!name.trim().is_empty(), TaskError::EmptyName);
        require!(!description.trim().is_empty(), TaskError::EmptyDescription);

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
