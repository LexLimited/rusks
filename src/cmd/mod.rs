pub mod cmd_executor;
pub mod execution;

use crate::core::task::TaskId;

pub enum Cmd {
    Version,
    Init,
    Deinit,
    Status,
    Add(AddCommand),
    Remove(TaskId),
    Edit(TaskId),
    List { pattern: String },
}

pub struct AddCommand {
    pub title: String,
    pub options: Vec<String>,
}
