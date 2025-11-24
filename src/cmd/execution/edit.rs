use std::{fs::File, io::Write, process};

use crate::{
    cmd::Error,
    core::task::{Task, TaskId},
    fs::{create_temp_file, open_temp_file, rusks_temp_file_relative_path},
    result::Result,
    storage::RusksStorage,
};

fn write_task_to_file(task: &Task, file: &mut File) -> Result<()> {
    let jstr = task.to_json().map_err(|e| Error::Reason {
        reason: format!("Failed to serialize task to json: {}", e),
    })?;

    file.write_all(jstr.as_bytes()).map_err(|e| Error::Reason {
        reason: format!(
            "Failed to write task json representation to temp file: {}",
            e
        ),
    })
}

fn edit(storage: &RusksStorage, id: TaskId, task: &mut Task) -> Result<()> {
    let temp_file_name = format!("edit_{id}");
    let mut temp_file = create_temp_file(&temp_file_name).map_err(|e| Error::Reason {
        reason: format!("Failed to create temp file: {}", e),
    })?;

    write_task_to_file(task, &mut temp_file)?;

    let status = process::Command::new("nvim")
        .arg(rusks_temp_file_relative_path(temp_file_name.as_str()))
        .status();

    match status {
        Ok(status) if status.success() => {
            let mut temp_file = open_temp_file(&temp_file_name)?;
            let task = Task::from_file(&mut temp_file)?;
            storage.change_task(id, &task)
        }
        _ => Err(Error::Generic),
    }
}

pub fn exec_edit(storage: &RusksStorage, id: TaskId) -> Result<()> {
    match storage.get_by_id(*id) {
        Some(mut item) => edit(storage, *id, item.get_task_mut()),
        None => Err(Error::Reason {
            reason: "Item not found".to_string(),
        }),
    }
}
