use clap::ArgMatches;

use crate::cmd::{CMD, Error};

pub fn create_edit(matches: &ArgMatches) -> Result<CMD, Error> {
    if let Some(id) = matches.get_one::<u64>("id") {
        return Ok(CMD::Edit{
            id: Some(*id),
            name: None
        })
    }

    Err(Error::Generic)
}
