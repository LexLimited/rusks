use crate::{core::task::Task, error::Error, storage::RusksStorage};

pub fn exec_add(storage: &RusksStorage, title: &String, _: &Vec<String>) -> Result<(), Error> {
    let task = Task::new(title);
    println!("Will add a new task: {}", task);

    storage.insert_task(&task)
}
