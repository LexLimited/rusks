use crate::{cmd::Cmd, context::Context, error::Error};

pub struct CmdExecutor<'a> {
    ctx: &'a Context,
}

impl<'a> CmdExecutor<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { ctx }
    }

    pub fn execute(&self, cmd: Cmd) -> Result<(), Error> {
        match cmd {
            Cmd::Init => Ok(()),
        }
    }
}
