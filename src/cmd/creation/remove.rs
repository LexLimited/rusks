use clap::ArgMatches;

use crate::{result::Result, error::Error, cmd::CMD};

pub fn create_remove(matches: &ArgMatches) -> Result<CMD> {
    match matches.get_one::<u64>("id") {
        Some(id) => Ok(CMD::Remove{
            id: Some(*id),
            name: None
        }),
        None => Err(Error::Reason{ reason: "Failed to parse an `id` argument".to_string() })
    }
}
