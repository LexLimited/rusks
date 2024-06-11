use crate::{error::Error, storage::RusksStorage};

pub fn exec_list(storage: &RusksStorage, pattern: &String) -> Result<(), Error> {
    for item in storage.get_all() {
        println!("{}", item.get_task().to_md()?)
    }

    Ok(())
}
