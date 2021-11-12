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

#[test]
fn sim_works() {
    let mut board = crate::board::Board::default();
    let width = board.dimensions.0;
    println!("{}", width);
    let display = |s: String, width: u32| {
        println!(
            "{}",
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
    };

    display(board.to_string(), width);
    board.update();
    display(board.to_string(), width);
}
