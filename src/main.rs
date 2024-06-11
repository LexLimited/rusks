mod algorithms;
mod cmd;
mod fs;
mod storage;
mod task;
mod prompt;
mod error;
mod result;

use cmd::CMD;
use result::Result;

fn run_cmd() -> Result<()> {
    CMD::new()
        ?.exec()
        .or_else(|e| {
            Ok(eprintln!("Command failed: {}", e))
        })
}

fn main() -> Result<()> {
    run_cmd()
}
