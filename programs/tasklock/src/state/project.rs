use anchor_lang::prelude::*;

#[account]
pub struct Project {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub task_count: u32,
    pub active_tasks: u32,
    pub completed_tasks: u32,
    pub created_at: i64,
    pub bump: u8,
}

impl Project {
    pub fn space(name: &str, descriptions: &str) -> usize {
        8 + // discriminator
        32 + // owner pubkey
        4 + name.len() + // name string
        4 + descriptions.len() + // description string
        4 + // task_count
        4 + // active_tasks
        4 + // completed_taks
        8 + // created_at
        1 // bump
    }
}
