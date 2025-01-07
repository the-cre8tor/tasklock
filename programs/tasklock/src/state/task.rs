use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub project: Pubkey,
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub assignee: Pubkey,
    pub status: TaskStatus,
    pub created_at: i64,
    pub deadline: i64,
    pub completed_at: Option<i64>,
    pub task_id: u32,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Created,
    InProgress,
    Completed,
}

impl Task {
    pub fn space(title: &str, description: &str) -> usize {
        8 + // discriminator
        4 + title.len() + // title
        4 + description.len() + // description
        32 + // creator
        32 + // assignee
        1 + // status
        8 + // created_at
        8 + // deadline
        16 + // completed_at
        4 + // task_id
        1 // bump
    }
}
