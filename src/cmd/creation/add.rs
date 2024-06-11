use clap::ArgMatches;

use crate::{error::Error, cmd::CMD};

pub fn create_add(matches: &ArgMatches) -> Result<CMD, Error> {
    if let Some(title) = matches.get_one::<String>("title") {
        return Ok(CMD::Add{
            title: title.clone(),
            options: vec![]
        })
    }

    Err(Error::Generic)
}
