use crate::state::Project;
use anchor_lang::prelude::*;

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
