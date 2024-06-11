use std::{process, io::Write};

use crate::{
    task::Task,
    fs::{
        rusks_temp_file_relative_path,
        create_temp_file}
};

use super::Error;

fn write_task_to_tmp_file(task: &Task, file_name: &str) -> Result<(), Error> {
    let mut file = create_temp_file(file_name).map_err(|e| Error::Reason{
        reason: format!("Failed to create temp file: {}", e)
    })?;

    let jstr = task.to_json().map_err(|e| Error::Reason{
        reason: format!("Failed to serialize task to json: {}", e)
    })?;

    file.write_all(jstr.as_bytes()).map_err(|e| Error::Reason{
        reason: format!("Failed to write task json representation to temp file: {}", e)
    })
}

pub fn edit(id: u64, task: &mut Task) -> Result<(), Error> {
    let temp_file_name = format!("edit_{}", id);
    
    write_task_to_tmp_file(task, &temp_file_name)?;

    let status = process::Command::new("nvim")
        .arg(rusks_temp_file_relative_path(temp_file_name.as_str()))
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        _ => Err(Error::Generic),
    }
}
