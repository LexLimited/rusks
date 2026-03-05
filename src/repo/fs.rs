use std::{
    fs::{create_dir, remove_dir_all},
    path::{Path, PathBuf},
};

use anyhow::Result;

pub(crate) struct FsRepositoryManager {}

impl FsRepositoryManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_repo(&self) -> bool {
        todo!()
    }

    pub fn init(&self) -> Result<()> {
        todo!()
    }

    pub fn deinit(&self) -> Result<()> {
        todo!()
    }
}

const RUSKS_DIRECTORY_RELATIVE: &'static str = "./.rusks";

fn rusks_directory_relative_path() -> &'static str {
    RUSKS_DIRECTORY_RELATIVE
}

fn rusks_storage_relative_path() -> PathBuf {
    Path::new(rusks_directory_relative_path()).join("storage")
}

fn rusks_directory_exists() -> bool {
    Path::new(rusks_directory_relative_path()).exists()
}

fn create_rusks_directory() -> Result<()> {
    Ok(create_dir(rusks_directory_relative_path())?)
}

fn remove_rusks_directory() -> Result<()> {
    Ok(remove_dir_all(rusks_directory_relative_path())?)
}

pub fn is_rusks_repository() -> bool {
    rusks_directory_exists()
}

pub fn init_rusks_repository() -> Result<()> {
    if is_rusks_repository() {
        return Ok(());
    }

    create_rusks_directory()
}

pub fn deinit_rusks_repository() -> Result<()> {
    remove_rusks_directory()
}
