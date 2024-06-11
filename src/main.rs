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
use storage::RusksStorage;

fn run_cmd(storage: &RusksStorage) -> Result<()> {
    CMD::new()
        ?.exec(storage)
        .or_else(|e| {
            Ok(eprintln!("Command failed: {}", e))
        })
}

fn main() -> Result<()> {
    let storage = RusksStorage::new()?;
    run_cmd(&storage)
}
