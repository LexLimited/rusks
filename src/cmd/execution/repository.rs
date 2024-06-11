use crate::{fs::{delete_rusks_repository, init_rusks_repository}, cmd::Error, prompt::confirm_and_run};

pub fn exec_init() -> Result<(), Error> {
    init_rusks_repository().map_err(|_| Error::Generic)
}

pub fn exec_delete() -> Result<(), Error> {
    match confirm_and_run("Delete rusks repository?", || {
        delete_rusks_repository().map_err(|e| Error::Reason{ reason: e.to_string() })
    }) {
        Some(r) => r,
        None => Ok(())
    }
}
