use crate::{result::Result, storage::RusksStorage};

pub struct Context {
    storage: RusksStorage,
}

impl Context {
    pub fn try_new() -> Result<Self> {
        Ok(Self {
            storage: RusksStorage::new()?,
        })
    }

    pub fn storage(&self) -> &RusksStorage {
        &self.storage
    }
}
