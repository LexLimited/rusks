pub mod cmd_executor;
mod execution;

use crate::{core::task::TaskId, error::Error};

pub enum Cmd {
    Init,
    Delete,
    Status,
    Add(Add),
    Remove(TaskId),
    Edit(TaskId),
    List { pattern: String },
}

pub struct Add {
    title: String,
    options: Vec<String>,
}

impl Add {
    pub fn new(title: String, options: Vec<String>) -> Self {
        Self { title, options }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
}
