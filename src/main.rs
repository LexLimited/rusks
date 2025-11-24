mod algorithms;
mod cli;
mod cmd;
mod context;
mod core;
mod error;
mod fs;
mod prompt;
mod result;
mod storage;

use result::Result;

use crate::{cli::CliParser, cmd::cmd_executor::CmdExecutor, context::Context};

fn main() -> Result<()> {
    CmdExecutor::new(&Context::try_new()?).execute(CliParser::get_cmd()?)
}
