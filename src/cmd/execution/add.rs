use crate::{error::Error, task::Task, storage::RusksStorage};

pub fn exec_add(storage: &RusksStorage, title: &String, options: &Vec<String>) -> Result<(), Error> {
    let task = Task::new(title);
    println!("Will add a new task: {}", task);

    storage.insert_task(&task)
}
