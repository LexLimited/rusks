use crate::{
    cmd::{Add, Cmd},
    context::Context,
    core::task::{Task, TaskId},
    result::Result,
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
            Cmd::Init => Ok(()),
            Cmd::Remove(cmd) => self.exec_remove(cmd),
            _ => todo!(),
        }
    }

    pub fn exec_add(&self, cmd: Add) -> Result<()> {
        let task = Task::new(cmd.title());
        println!("Will add a new task: {}", task);

        self.ctx.storage().insert_task(&task)
    }

    fn exec_remove(&self, id: TaskId) -> Result<()> {
        self.ctx.storage().remove_by_id(id)
    }
}
