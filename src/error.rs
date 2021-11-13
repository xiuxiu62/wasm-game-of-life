use std::{error, io, result};

#[derive(Debug)]
pub enum GameError {
    BoardInitializationError(String),
    CellDoesNotExist(String),
    Io(io::Error),
    Other(Box<dyn error::Error>),
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Box<dyn error::Error>> for GameError {
    fn from(err: Box<dyn error::Error>) -> Self {
        Self::Other(err)
    }
}

pub type Result<T> = result::Result<T, GameError>;
