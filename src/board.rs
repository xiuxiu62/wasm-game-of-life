use std::fmt;

use crate::{
    cell::{Cell, Coordinates, NEIGHBOUR_OPS},
    error::{GameError, Result},
};

#[derive(Debug)]
pub struct Board {
    pub dimensions: (u32, u32),
    pub cells: Vec<Cell>,
    size: usize,
    debug: bool,
}

impl Board {
    pub fn new(dimensions: (u32, u32), cells: Vec<Cell>, debug: bool) -> Result<Self> {
        let size = (dimensions.0 * dimensions.1) as usize;
        if size != cells.len() {
            Err(GameError::BoardInitializationError(format!(
                "Board was supposed to be of size {}, but received one of size {}",
                size,
                cells.len()
            )))
        } else {
            Ok(Self {
                dimensions,
                cells,
                size,
                debug,
            })
        }
    }

    pub fn update(&mut self) -> Result<()> {
        (0..self.size).try_for_each(|i| self.update_cell(i as u32))
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn update_cell(&mut self, index: u32) -> Result<()> {
        let cell = *self.get_cell(index)?;
        let neighbors = self.get_neighbors(index)?;
        let neighbors_living = neighbors.iter().filter(|n| n.is_alive()).count();

        match cell {
            Cell::Dead => {
                if neighbors_living == 3 {
                    self.cells[index as usize] = Cell::Alive;
                }
            }
            Cell::Alive => {
                if let 0..=1 | 4.. = neighbors_living {
                    self.cells[index as usize] = Cell::Dead;
                }
            }
        };

        Ok(())
    }

    fn get_cell(&self, index: u32) -> Result<&Cell> {
        match self.cells.get(index as usize) {
            Some(cell) => Ok(cell),
            None => Err(GameError::CellDoesNotExist(format!(
                "Cell of index {} does not exist",
                index
            ))),
        }
    }

    fn convert_to_2d(&self, index: u32) -> Coordinates {
        let width = self.dimensions.0;
        let x = (index % width) as i64;
        let y = (index / width) as i64;

        Coordinates(x, y)
    }

    fn convert_to_1d(&self, &Coordinates(x, y): &Coordinates) -> u32 {
        (y as u32 * self.dimensions.0) + (x as u32)
    }

    fn get_neighbor_coords(&self, index: u32) -> Vec<Coordinates> {
        let coords = self.convert_to_2d(index);
        NEIGHBOUR_OPS
            .into_iter()
            .map(|op| coords - op)
            .filter(|Coordinates(x, y)| {
                *x > -1 && *y > -1 && *x < self.dimensions.0 as i64 && *y < self.dimensions.1 as i64
            })
            .collect::<Vec<Coordinates>>()
    }

    fn get_neighbors(&self, index: u32) -> Result<Vec<&Cell>> {
        let neighbor_coords = self.get_neighbor_coords(index);
        neighbor_coords
            .iter()
            .map(|coords| self.convert_to_1d(coords))
            .map(|index| self.get_cell(index))
            .collect::<Result<Vec<&Cell>>>()
    }
}

impl Default for Board {
    fn default() -> Self {
        let dimensions = (128, 64);
        let size = (dimensions.0 * dimensions.1) as usize;
        let cells: Vec<Cell> = (0..size)
            .into_iter()
            .map(|i| if i % 2 == 0 { Cell::Alive } else { Cell::Dead })
            .collect();

        Self {
            dimensions,
            cells,
            size,
            debug: false,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .fold("".to_string(), |acc, cell| acc + &cell.to_string())
        )
    }
}
