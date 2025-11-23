pub mod cmd_executor;
mod execution;

use crate::error::Error;

pub enum Cmd {
    Init,
    Delete,
    Status,
    Add {
        title: String,
        options: Vec<String>,
    },
    Remove {
        id: Option<u64>,
        name: Option<String>,
    },
    Edit {
        id: Option<u64>,
        name: Option<String>,
    },
    List {
        pattern: String,
    },
}
