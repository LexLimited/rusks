mod repository;
mod status;
mod new;
mod edit;
mod list;

use crate::{fs::is_rusks_repository, storage::RusksStorage};

use self::{
    edit::exec_edit,
    repository::{exec_init, exec_delete}, status::exec_status, new::exec_new, list::exec_list
};

use super::{CMD, Error};

impl CMD {
    pub fn exec(&self, storage: &RusksStorage) -> Result<(), Error> {
        if !is_rusks_repository() {
            if let CMD::Init = self {
                return exec_init();
            }
            return Err(Error::Reason{ reason: "Not a rusks repository".to_string() });
        }

        match self {
            CMD::Delete => exec_delete(),
            CMD::List { pattern } => exec_list(storage, pattern),
            CMD::Status => exec_status(),
            CMD::New { title, options } => exec_new(storage, title, options),
            CMD::Edit { id, name } => exec_edit(storage, id, name),
            CMD::Remove { id, name } => todo!("implement remove"),
            _ => Err(Error::Reason{ reason: "Unhandled".to_string() })
        }
    }
}
