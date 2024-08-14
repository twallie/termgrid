use std::{fmt::Display, io::{self, Write}};

use crate::grid::Grid;

pub enum PrinterUpdateError {
    NoPreviousState
}

pub struct Printer<T> {
    current_state: Grid<T>,
    previous_state: Option<Grid<T>>
}

impl <T> Printer<T>
where T: Copy + Display + Eq {
    pub fn new(state: Grid<T>) -> Self {
        Self {
            current_state: state,
            previous_state: None
        }
    }

    pub fn start(&self) -> () {
        initial_print(self.current_state.clone());
    }

    pub fn update(mut self, new_grid: Grid<T>) -> Result<Self, PrinterUpdateError> {
        self.previous_state = Some(self.current_state);
        self.current_state = new_grid;

        update_print(
            &self.current_state, 
            match &self.previous_state {
                Some(v) => v,
                None => return Err(PrinterUpdateError::NoPreviousState)
            }
        );

        Ok(self)
    }
    
}

fn initial_print<T>(grid: Grid<T>) -> ()
where T: Copy + Display {
    let col_count = grid.height();
    let row_count = grid.height();
    print!("{}{}", termion::clear::All, termion::cursor::Hide);
    for y in 0..col_count {
        for x in 0..row_count {
            let cell_state = grid.get(x, y);
            let terminal_coordinates = convert_grid_coordinates_to_terminal_coordinates(
                x, 
                y, 
                grid.height()
            );
            print!(
                "{}{}",
                termion::cursor::Goto(
                    terminal_coordinates.0 as u16, 
                    terminal_coordinates.1 as u16
                ),
                cell_state
            )
        }
    }

    let _ = io::stdout().flush();
}

fn convert_grid_coordinates_to_terminal_coordinates(grid_x: usize, grid_y: usize, grid_height: usize) -> (usize, usize) {
    let terminal_x = grid_x + 1;
    let terminal_y = grid_height - grid_y;

    (terminal_x, terminal_y)
}

fn update_print<T>(grid: &Grid<T>, previous_grid: &Grid<T>) -> ()
where T: Copy + Display + Eq {
    let col_count = grid.height();
    let row_count = grid.length();
    for y in 0..col_count {
        for x in 0..row_count {
            let cell_state = grid.get(x, y);
            let prev_state = previous_grid.get(x, y);
            let terminal_coordinates = convert_grid_coordinates_to_terminal_coordinates(
                x,
                y,
                grid.length()
            );

            if cell_state != prev_state {
                print!(
                    "{}{}",
                    termion::cursor::Goto(
                        terminal_coordinates.0 as u16,
                        terminal_coordinates.1 as u16
                    ),
                    cell_state
                )
            }
        }
    }
    let _ = io::stdout().flush();
}
