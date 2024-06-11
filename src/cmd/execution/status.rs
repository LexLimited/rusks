use crate::{error::Error, storage::RusksStorage};

pub fn exec_status(storage: &RusksStorage) -> Result<(), Error> {
    match storage.get_all().len() {
        n_tasks if n_tasks > 0 => Ok(println!("{} unfinished tasks", n_tasks)),
        _ => Ok(println!("Rusks has no tasks"))
    }
}
