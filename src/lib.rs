#![allow(dead_code)]

mod board;
mod cell;
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
    use std::{
        io::{self, Write},
        thread, time,
    };

    use crate::{board::Board, error::Result};

    fn display(stdout: &mut io::Stdout, board: &Board) -> Result<()> {
        let s: String = board.to_string();
        let width: u32 = board.dimensions.0;

        let message = format!(
            "{}\n",
            s.chars().enumerate().fold("".to_string(), |acc, (i, c)| {
                if i > 0 {
                    acc + &match i as u32 % width {
                        0 => format!("\n{}", c.to_string()),
                        _ => c.to_string(),
                    }
                } else {
                    acc + &c.to_string()
                }
            })
        );
        stdout.write(message.as_bytes())?;
        Ok(())
    }

    fn cycle(stdout: &mut io::Stdout, board: &mut Board) -> Result<()> {
        display(stdout, board)?;
        board.update()
    }

    #[test]
    fn sim_works() -> Result<()> {
        let mut board = Board::default();
        let mut stdout = io::stdout();
        let duration = time::Duration::from_millis(50);

        print!("\x1B[2J\x1B[1;1H");
        for _ in 0..500 {
            cycle(&mut stdout, &mut board)?;
            thread::sleep(duration);
            print!("\x1B[2J\x1B[1;1H");
        }

        Ok(())
    }
}
