mod algorithms;
mod cli;
mod cmd;
mod context;
mod core;
mod prompt;
mod repo;
mod storage;
mod version;

use anyhow::Result;

use crate::{cli::CliParser, cmd::cmd_executor::CmdExecutor, context::Context};

fn main() -> Result<()> {
    CmdExecutor::new(&Context::try_new()?).execute(CliParser::get_cmd()?)
}
