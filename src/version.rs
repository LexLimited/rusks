pub const VERSION_CODE: &'static str = "0.0.1";

#[cfg(feature = "fs-repo")]
pub const VERSION_REPO: &'static str = "file system";

#[cfg(not(feature = "fs-repo"))]
pub const VERSION_REPO: &'static str = "virtual";
