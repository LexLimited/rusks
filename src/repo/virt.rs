use std::cell::RefCell;

use anyhow::{anyhow, Result};

pub(crate) struct VirtualRepositoryManager {
    is_initialized: RefCell<bool>,
}

impl VirtualRepositoryManager {
    pub fn new() -> Self {
        Self {
            is_initialized: RefCell::new(false),
        }
    }

    pub fn is_repo(&self) -> bool {
        self.is_initialized.borrow().clone()
    }

    pub fn init(&self) -> Result<()> {
        if self.is_repo() {
            return Err(anyhow!("already initialized"));
        }

        self.is_initialized.replace(true);
        Ok(())
    }

    pub fn deinit(&self) -> Result<()> {
        if !self.is_repo() {
            return Err(anyhow!("not a rusks repo"));
        }

        self.is_initialized.replace(false);
        Ok(())
    }
}
