mod repository;
mod status;
mod add;
mod edit;
mod remove;
mod list;

use crate::{fs::is_rusks_repository, storage::RusksStorage};

use self::{
    edit::exec_edit,
    repository::{exec_init, exec_delete}, status::exec_status, add::exec_add, list::exec_list, remove::exec_remove
};

use super::{CMD, Error};

impl CMD {
    pub fn exec(&self) -> Result<(), Error> {
        if !is_rusks_repository() {
            if let CMD::Init = self {
                return exec_init();
            }
            return Err(Error::Reason{ reason: "Not a rusks repository".to_string() });
        }

        let storage = &RusksStorage::new()?;

        match self {
            CMD::Delete => exec_delete(),
            CMD::List { pattern } => exec_list(storage, pattern),
            CMD::Status => exec_status(storage),
            CMD::Add { title, options } => exec_add(storage, title, options),
            CMD::Edit { id, name } => exec_edit(storage, id, name),
            CMD::Remove { id, name } => exec_remove(storage, id, name),
            _ => Err(Error::Reason{ reason: "Unhandled".to_string() })
        }
    }
}
