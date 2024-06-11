use std::fmt;

use sled::Db;

use crate::{
    task::Task,
    fs::rusks_storage_relative_path, error::Error, result::Result
};

pub struct RusksStorage {
    db: Db,
}

pub struct Item {
    id: u64,
    task: Task,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "> TASK (id: {})\n", self.id)?;
        write!(f, "{}", self.task)
    }
}

impl Item {
    pub fn get_id(&self) -> u64 {
        self.id
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
        let db = sled::open(
            rusks_storage_relative_path()    
        )?;
        Ok(RusksStorage{ db })
    }

    fn generate_key(&self) -> sled::Result<[u8; 8]> {
        Ok(self.db.generate_id()?.to_le_bytes())
    }

    pub fn insert_task(&self, task: &Task) -> Result<()> {
        let key  = self.generate_key()?;
       
        let v = task.to_vec()?;
        if let Ok(_) = self.db.insert(key, v.as_slice()) {
            return Ok(())
        }

        Err(Error::Generic)
    }

    pub fn change_task(&self, id: u64, task: &Task) -> Result<()> {
        let v = task.to_vec()?;

        if self.db.get(id.to_le_bytes())?.is_none() {
            return Err(Error::Reason{
                reason: "Task does not exist or failed to find it".to_string()
            })
        }

        if let Ok(_) = self.db.insert(id.to_le_bytes(), v.as_slice()) {
            return Ok(())
        }
        
        Err(Error::Generic)
    }

    pub fn get_by_id(&self, id: u64) -> Option<Item> {
        match self.db.get(id.to_le_bytes()) {
            Ok(v) => match v {
                Some(v) => {
                    if let Ok(task) = Task::from_bytes(&v) {
                        return Some(Item{ id, task })
                    }
                    return None
                },
                None => None
            },
            Err(_) => None
        }
    }

    pub fn get_all(&self) -> Vec<Item> {
        let mut ret: Vec<Item> = Vec::new();

        for p in self.db.iter() {
            if let Ok((k, v)) = p {
                let id_bytes: &[u8] = &k;
                let id = u64::from_le_bytes(id_bytes.try_into().expect("Wrong size of id"));
                let task = Task::from_bytes(&v);

                if let Ok(task) = task {
                    ret.push(Item{ id, task });
                }
            }
        }

        ret
    }

    pub fn remove_by_id(&self, id: u64) -> Result<()> {
        if let Err(e) = self.db.remove(id.to_le_bytes()) {
            return Err(Error::Reason{ reason: format!("Failed to remove an item: {}", e) });
        }
        Ok(())
    }
}
