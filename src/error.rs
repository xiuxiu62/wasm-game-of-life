use std::{error, result};

#[derive(Debug)]
pub enum GameError {
    BoardInitializationError(String),
    CellDoesNotExist(String),
    Other(Box<dyn error::Error>),
}

impl From<Box<dyn error::Error>> for GameError {
    fn from(err: Box<dyn error::Error>) -> Self {
        Self::Other(err)
    }
}

pub type Result<T> = result::Result<T, GameError>;
