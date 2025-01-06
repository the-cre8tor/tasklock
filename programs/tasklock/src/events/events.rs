use anchor_lang::prelude::*;

#[event]
pub struct ProjectCreated {
    pub project: Pubkey,
    pub owner: Pubkey,
    pub name: String,
}

#[event]
pub struct TaskCreated {
    pub project: Pubkey,
    pub task: Pubkey,
    pub creator: Pubkey,
    pub title: String,
}

#[event]
pub struct TaskAssigned {
    pub project: Pubkey,
    pub task: Pubkey,
    pub assignee: Pubkey,
}

#[event]
pub struct TaskCompleted {
    pub project: Pubkey,
    pub task: Pubkey,
    pub completed_by: Pubkey,
}
