use cmd::{CMD, Error};

mod task;
mod help;
mod cmd;
mod storage;
mod algorithms;
mod fs;

fn run_cmd() -> Result<(), Error> {
    CMD::new()
        ?.exec()
        .or_else(|_| {
            Ok(println!("Command failed, type `rusks help` for help"))
        })
}

fn main() {
    run_cmd().ok();
}
