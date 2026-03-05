use std::{fs::File, io::Write, process};

use anyhow::{anyhow, Result};
use tempfile::NamedTempFile;

use crate::{
    core::task::{Task, TaskId},
    storage::RusksStorage,
};

pub fn exec_edit(storage: &RusksStorage, id: TaskId) -> Result<()> {
    match storage.get_by_id(id.clone()) {
        Some(mut item) => edit(storage, id, item.get_task_mut()),
        None => Err(anyhow!("Item not found")),
    }
}

fn edit(storage: &RusksStorage, id: TaskId, task: &mut Task) -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    write_task_to_file(task, &mut temp_file.as_file_mut())?;

    let status = process::Command::new("nvim").arg(temp_file.path()).status();

    match status {
        Ok(status) if status.success() => {
            let task = Task::from_file(&mut temp_file.into_file())?;
            storage.change_task(id, &task)
        }
        _ => Err(anyhow!("?")),
    }
}

fn write_task_to_file(task: &Task, file: &mut File) -> Result<()> {
    let jstr = task.to_json()?;
    file.write_all(jstr.as_bytes())?;

    Ok(())
}
