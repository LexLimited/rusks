use std::{io, fs::{
    create_dir,
    create_dir_all,
    remove_dir_all,
    File, remove_file, OpenOptions}, path::{PathBuf, Path}
};

const RUSKS_DIRECTORY_RELATIVE: &'static str = "./.rusks";

pub fn rusks_directory_relative_path() -> &'static str {
    RUSKS_DIRECTORY_RELATIVE
}

pub fn rusks_storage_relative_path() -> PathBuf {
    Path::new(rusks_directory_relative_path()).join("storage")
}

pub fn rusks_temp_relative_path() -> PathBuf {
    Path::new(rusks_directory_relative_path()).join("tmp")
}

fn rusks_directory_exists() -> bool {
    Path::new(rusks_directory_relative_path()).exists()
}

fn create_rusks_directory() -> io::Result<()> {
    create_dir(rusks_directory_relative_path())
}

fn remove_rusks_directory() -> io::Result<()> {
    remove_dir_all(rusks_directory_relative_path())
}

fn create_rusks_temp_directory() -> io::Result<()> {
    create_dir_all(rusks_temp_relative_path())
}

pub fn rusks_temp_file_relative_path(f_name: &str) -> PathBuf {
    rusks_temp_relative_path().join(f_name)
}

pub fn create_temp_file(name: &str) -> io::Result<File> {
    File::create(rusks_temp_file_relative_path(name))
}

pub fn open_temp_file(name: &str) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(rusks_temp_file_relative_path(name))
}

pub fn remove_temp_file(name: &str) -> io::Result<()> {
    remove_file(rusks_temp_file_relative_path(name))
}

pub fn is_rusks_repository() -> bool {
    rusks_directory_exists()
}

pub fn init_rusks_repository() -> io::Result<()> {
    if is_rusks_repository() {
        return Ok(())
    }

    create_rusks_directory()?;
    create_rusks_temp_directory()
}

pub fn delete_rusks_repository() -> io::Result<()> {
    remove_rusks_directory()
}
