use crate::cms::site::Site;
use std::fmt::{self, Debug, Formatter};

#[cfg(feature = "kirby")]
pub mod kirby;
//pub mod mongodb;
//pub mod redis;
//pub mod sqlite;

pub trait Database: Send + Sync + Debug {
    fn load(&self, site: &mut Site) -> Result<(), DatabaseError>;
    // refresh
    // save
}

pub struct DatabaseBuilder {}

impl DatabaseBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Box<dyn Database> {
        #[cfg(feature = "kirby")]
        Box::new(kirby::Kirby {})
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    IoError(std::io::Error),
    PathError(String),
    OtherError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::IoError(e) => write!(f, "IO error: {}", e),
            DatabaseError::PathError(e) => write!(f, "Path error: {}", e),
            DatabaseError::OtherError(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> DatabaseError {
        DatabaseError::IoError(err)
    }
}
