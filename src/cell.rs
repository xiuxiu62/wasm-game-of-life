use std::{fmt, ops};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Coordinates(pub i64, pub i64);

impl ops::Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub static NEIGHBOUR_OPS: [Coordinates; 8] = [
    Coordinates(-1, -1),
    Coordinates(-1, 0),
    Coordinates(-1, 1),
    Coordinates(0, -1),
    Coordinates(0, 1),
    Coordinates(1, -1),
    Coordinates(1, 0),
    Coordinates(1, 1),
];

// TODO: ensure u8 is derived from order
#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        self == &Cell::Alive
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self == &Cell::Dead { '◻' } else { '◼' })
    }
}
