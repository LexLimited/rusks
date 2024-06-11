use clap::ArgMatches;

use crate::{error::Error, cmd::CMD};

pub fn create_new(matches: &ArgMatches) -> Result<CMD, Error> {
    if let Some(title) = matches.get_one::<String>("title") {
        return Ok(CMD::New{
            title: title.clone(),
            options: vec![]
        })
    }

    Err(Error::Generic)
}
