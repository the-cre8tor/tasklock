use anchor_lang::prelude::error_code;

#[error_code]
pub enum TaskError {
    #[msg("Name must be 50 characters or less")]
    NameTooLong,
    #[msg("Description must be 200 characters or less")]
    DescriptionTooLong,
    #[msg("Title must be 50 characters or less")]
    TitleTooLong,
    #[msg("Deadline must be in the future")]
    InvalidDeadline,
    #[msg("Maximum number of tasks reached")]
    MaxTasksReached,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Task is already completed")]
    TaskAlreadyCompleted,
    #[msg("Unauthorized to assign task")]
    UnauthorizedAssignment,
    #[msg("Unauthorized to complete task")]
    UnauthorizedCompletion,
}