use serde::{Deserialize, Serialize};
use validator::Validate;

pub const FIELD_TASK: &str = "task";

#[derive(Deserialize, Validate)]
pub struct CreateTodoForm {
    #[validate(length(min = 1, message = "Task cannot be empty"))]
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: crate::models::TodoId,
    pub task: String,
    pub is_done: bool,
}
