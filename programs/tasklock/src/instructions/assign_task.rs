use anchor_lang::prelude::*;

use crate::state::{Project, Task};

/// This is an Account Validation Struct which defines what accounts are required and how they should be validated for a particular instruction.
/// This struct specifically defines the account constraints and validation rules that must be met when executing the assign_task instruction. Let's break down what it does:
///
/// The struct defines three required accounts:
///
/// 1. owner: A mutable signer account
/// 2. project: A Project account that must be derived from PDA with specific seeds
/// 3. task: A mutable Task account that must be derived from PDA with specific seeds
///
/// The actual instruction would be the function that uses this account struct, which would look something like:
/// ```rust
/// pub fn assign_task(ctx: Context<AssignTask>, new_assignee: Pubkey) -> Result<()> {
///     // Instruction logic here
/// }
/// ```
#[derive(Accounts)]
pub struct AssignTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [b"project", owner.key().as_ref()],
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
