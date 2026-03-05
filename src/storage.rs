use std::fmt;

use anyhow::{anyhow, Result};
use sled::Db;

use crate::core::task::{Task, TaskId};

pub struct RusksStorage {
    db: Db,
}

pub struct Item {
    id: TaskId,
    task: Task,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "> TASK (id: {})\n", self.id)?;
        write!(f, "{}", self.task)
    }
}

impl Item {
    pub fn id(&self) -> TaskId {
        self.id.clone()
    }

    pub fn get_task(&self) -> &Task {
        &self.task
    }

    pub fn get_task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
}

impl RusksStorage {
    /// Creates a new instance ot RusksStorage.
    /// Opens a sled db, which shoul only happen once.
    pub fn new() -> Result<Self> {
        let db = sled::open("./.rusks/storage")?;
        Ok(RusksStorage { db })
    }

    fn generate_key(&self) -> sled::Result<[u8; 8]> {
        Ok(self.db.generate_id()?.to_le_bytes())
    }

    pub fn insert_task(&self, task: &Task) -> Result<()> {
        self.db
            .insert(self.generate_key()?, task.to_vec()?.as_slice())?;

        Ok(())
    }

    pub fn change_task(&self, id: TaskId, task: &Task) -> Result<()> {
        let v = task.to_vec()?;

        if self.db.get(id.as_bytes())?.is_none() {
            return Err(anyhow!("Task does not exist"));
        }

        if let Ok(_) = self.db.insert(id.as_bytes(), v.as_slice()) {
            return Ok(());
        }

        Err(anyhow!("?"))
    }

    pub fn get_by_id(&self, id: TaskId) -> Option<Item> {
        match self.db.get(id.as_bytes()) {
            Ok(v) => match v {
                Some(v) => {
                    if let Ok(task) = Task::from_bytes(&v) {
                        return Some(Item { id, task });
                    }
                    return None;
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn get_all(&self) -> Result<Vec<Item>> {
        let mut ret: Vec<Item> = Vec::new();

        for p in self.db.iter() {
            if let Ok((k, v)) = p {
                let id = TaskId::try_from_bytes(&k)?;
                let task = Task::from_bytes(&v);

                if let Ok(task) = task {
                    ret.push(Item { id, task });
                }
            }
        }

        Ok(ret)
    }

    pub fn remove_by_id(&self, id: TaskId) -> Result<()> {
        if let Err(e) = self.db.remove(id.as_bytes()) {
            return Err(anyhow!("Failed to remove an item: {}", e));
        }
        Ok(())
    }

    fn id_from_key(key: &[u8]) -> Result<TaskId> {
        Ok(TaskId::new(u64::from_le_bytes(key.try_into()?)))
    }

    fn key_from_id<'a>(id: &'a TaskId) -> &'a [u8] {
        id.as_bytes()
    }
}
