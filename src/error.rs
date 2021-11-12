use std::result;

pub enum GameError {
    BoardInitializationError(String),
}

pub type Result<T> = result::Result<T, GameError>;
