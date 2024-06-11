use crate::{storage::RusksStorage, result::Result, error::Error};

pub fn exec_remove(storage: &RusksStorage, id: &Option<u64>, name: &Option<String>) -> Result<()> {
    if let Some(_) = name {
        return Err(Error::Reason{ reason: "Remove by name is not yet supported".to_string() });
    }

    match id {
        Some(id) => storage.remove_by_id(*id),
        None => Err(Error::Reason{ reason: "`id` was not specified for `remove`".to_string() })
    }
}
