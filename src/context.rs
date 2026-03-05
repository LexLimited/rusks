use anyhow::Result;

use crate::{repo::RepositoryManager, storage::RusksStorage};

pub struct Context {
    storage: RusksStorage,
    pub rep_mgr: RepositoryManager,
}

impl Context {
    pub fn try_new() -> Result<Self> {
        Ok(Self {
            storage: RusksStorage::new()?,
            rep_mgr: RepositoryManager::new(),
        })
    }

    pub fn storage(&self) -> &RusksStorage {
        &self.storage
    }
}
