pub mod fs;
pub mod virt;

use anyhow::Result;

#[cfg(feature = "fs-repo")]
use crate::repo::fs::FsRepositoryManager;

#[cfg(not(feature = "fs-repo"))]
use crate::repo::virt::VirtualRepositoryManager;

pub struct RepositoryManager {
    inner: RepositoryManagerImpl,
}

impl RepositoryManager {
    pub fn new() -> Self {
        Self {
            inner: RepositoryManagerImpl::new(),
        }
    }

    pub fn is_repo(&self) -> bool {
        self.inner.is_repo()
    }

    pub fn init(&self) -> Result<()> {
        self.inner.init()
    }

    pub fn deinit(&self) -> Result<()> {
        self.inner.deinit()
    }
}

#[cfg(feature = "fs-repo")]
type RepositoryManagerImpl = FsRepositoryManager;

#[cfg(not(feature = "fs-repo"))]
type RepositoryManagerImpl = VirtualRepositoryManager;
