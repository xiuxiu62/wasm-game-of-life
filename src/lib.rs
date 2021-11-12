#![allow(dead_code)]

mod board;
mod error;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("hello {}", name).as_str());
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, error::Result};

    fn display(s: String, width: u32) {
        println!(
            "{}\n",
            s.chars().enumerate().fold("".to_string(), |acc, (i, c)| {
                if i > 0 {
                    acc + &match i as u32 % width {
                        0 => format!("{}\n", c.to_string()),
                        _ => c.to_string(),
                    }
                } else {
                    acc + &c.to_string()
                }
            })
        )
    }

    fn cycle(board: &mut Board) -> Result<()> {
        let width = board.dimensions.0;
        display(board.to_string(), width);
        board.update()
    }

    #[test]
    fn sim_works() -> Result<()> {
        let mut board = Board::default();

        display(board.to_string(), board.dimensions.0);
        board.update()?;
        display(board.to_string(), board.dimensions.0);
        // for _ in 0..3 {
        //     cycle(&mut board)?;
        // }

        Ok(())
    }
}
