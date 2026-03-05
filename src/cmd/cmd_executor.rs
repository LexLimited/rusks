use std::{io::Write, process};

use anyhow::Result;
use tempfile::NamedTempFile;

use crate::{
    cmd::{execution::edit::exec_edit, AddCommand, Cmd},
    context::Context,
    core::task::{Task, TaskId},
    prompt::confirm_and_run,
    version,
};

pub struct CmdExecutor<'a> {
    ctx: &'a Context,
}

impl<'a> CmdExecutor<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { ctx }
    }

    pub fn execute(&self, cmd: Cmd) -> Result<()> {
        match cmd {
            Cmd::Version => self.exec_version(),
            Cmd::Init => self.exec_init(),
            Cmd::Deinit => self.exec_deinit(),
            Cmd::Remove(task_id) => self.exec_remove(task_id),
            Cmd::Status => self.exec_status(),
            Cmd::Add(add) => self.exec_add(add),
            Cmd::Edit(task_id) => self.exec_edit(task_id),
            Cmd::List { .. } => self.exec_list(),
        }
    }

    pub fn exec_version(&self) -> Result<()> {
        println!(
            "Rusks {}, {} repository",
            version::VERSION_CODE,
            version::VERSION_REPO
        );

        Ok(())
    }

    pub fn exec_init(&self) -> Result<()> {
        self.ctx.rep_mgr.init()
    }

    fn exec_deinit(&self) -> Result<()> {
        match confirm_and_run("Delete rusks repository?", || self.ctx.rep_mgr.deinit()) {
            Some(r) => r,
            None => Ok(()),
        }
    }

    fn exec_add(&self, add_command: AddCommand) -> Result<()> {
        let task = Task::new(&add_command.title);
        println!("Will add a new task: {}", task);

        self.ctx.storage().insert_task(&task)
    }

    fn exec_remove(&self, id: TaskId) -> Result<()> {
        self.ctx.storage().remove_by_id(id)
    }

    fn exec_status(&self) -> Result<()> {
        match self.ctx.storage().get_all()?.len() {
            n_tasks if n_tasks > 0 => Ok(println!("{} unfinished tasks", n_tasks)),
            _ => Ok(println!("Rusks has no tasks")),
        }
    }

    fn exec_edit(&self, task_id: TaskId) -> Result<()> {
        exec_edit(self.ctx.storage(), task_id)
    }

    fn exec_list(&self) -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        for item in self.ctx.storage().get_all()? {
            if let Ok(md_str) = item.get_task().to_md() {
                temp_file.write_all(format!("(id = {}):\n", item.id()).as_bytes())?;
                if let Err(e) = temp_file.write_all(md_str.as_bytes()) {
                    eprintln!("Failed to write to fancy file: {}", e);
                }
            }
        }

        process::Command::new("glow")
            .arg(temp_file.path())
            .status()?;

        Ok(())
    }
}
