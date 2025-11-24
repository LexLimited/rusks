use std::{fs::OpenOptions, io::Write, process};

use crate::{
    error::Error,
    fs::{create_temp_file, rusks_temp_relative_path},
    result::Result,
    storage::RusksStorage,
};

pub fn exec_list(storage: &RusksStorage, pattern: &String) -> Result<()> {
    let path = rusks_temp_relative_path().join("fancy.md");
    let path_str = match path.to_str() {
        Some(s) => s,
        None => {
            return Err(Error::Reason {
                reason: "Failed to construct a path to temp file".to_string(),
            })
        }
    };

    create_temp_file("fancy.md").map_err(|e| Error::Reason {
        reason: format!("Failed to construct temp .md file: {}", e),
    })?;

    let mut temp_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path_str)
        .map_err(|e| Error::Reason {
            reason: format!("Failed to open temp file: {}", e),
        })?;

    for item in storage.get_all() {
        if let Ok(md_str) = item.get_task().to_md() {
            temp_file.write_all(format!("(id = {}):\n", item.id()).as_bytes());
            if let Err(e) = temp_file.write_all(md_str.as_bytes()) {
                eprintln!("Failed to write to fancy file: {}", e);
            }
        }
    }

    process::Command::new("glow").arg(path_str).status();

    Ok(())
}
