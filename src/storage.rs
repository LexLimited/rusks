use std::fmt;

use sled::{Db, Error};

use crate::{task::Task, fs::rusks_directory_relative_path};

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
    pub fn get_task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
}

impl RusksStorage {
    pub fn new() -> Result<Self, Error> {
        let db = sled::open(
            format!("{}/storage", rusks_directory_relative_path())
        )?;
        Ok(RusksStorage{ db })
    }

    /*
    pub fn insert_task(&self, task: &Task) -> Result<(), Error> {
        let id  = self.db.generate_id()?;
        self.db.insert(id, task)
    }
    */

    pub fn get_by_id(&self, id: u64) -> Option<Item> {
        let mut task = Task::new(&format!("Sample task {}", id));
        task
            .set_description("sample desceiption")
            .add_note("Note 1")
            .add_note("Note 2");

        Some(Item{ id, task })
    }

    pub fn get_all(&self) -> Vec<Item> {
        let mut ret = vec![];
        for id in 1..10 {
            let mut task = Task::new(&format!("Task {}", id));
            task
                .set_description("This is a description")
                .add_note("Message 1")
                .add_note("Message 2")
                .add_note("Message 3");
            ret.push(Item{ id, task })
        }

        ret
    }
}
