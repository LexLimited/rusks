use crate::{error::Error, storage::RusksStorage};

pub fn exec_status() -> Result<(), Error> {
    let n_tasks = RusksStorage::new()?.get_all().len();
    Ok(println!("{} unfinished tasks", n_tasks))
}
