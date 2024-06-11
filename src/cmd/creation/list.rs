use clap::ArgMatches;

use crate::{error::Error, cmd::CMD};

pub fn create_list(matches: &ArgMatches) -> Result<CMD, Error> {
    Ok(CMD::List {
        pattern: String::new()
    })
}
