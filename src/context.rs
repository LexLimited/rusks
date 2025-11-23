use sled::Db;

use crate::result;

pub struct Context {
    db: Db,
}

impl Context {
    pub fn try_new() -> result::Result<Self> {
        let db = sled::open("")?;
        Ok(Self { db })
    }
}
