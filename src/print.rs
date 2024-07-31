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

fn initial_print<T>(state: Grid<T>) -> () 
where T: Copy + Display {
    let col_count = state.grid.len();
    let row_count = state.grid[0].len();
    print!("{}{}", termion::clear::All, termion::cursor::Hide);
    for col in 1..col_count+1 {
        for row in 1..row_count+1 {
            let cell_state = state.grid[col-1][row-1];
            print!(
                "{}{}",
                termion::cursor::Goto(row as u16, col as u16),
                cell_state
            )
        }
    }

    print!(
        "{}",
        termion::cursor::Goto(0, row_count as u16)
    );
    let _ = io::stdout().flush();
}

fn update_print<T>(state: &Grid<T>, previous_state: &Grid<T>) -> ()
where T: Copy + Display + Eq {
    let col_count = state.grid.len();
    let row_count = state.grid[0].len();
    for col in 1..col_count+1 {
        for row in 1..row_count+1 {
            let cell_state = state.grid[col-1][row-1];
            let prev_state = previous_state.grid[col-1][row-1];
            if cell_state != prev_state {
                print!(
                    "{}{}",
                    termion::cursor::Goto(row as u16, col as u16),
                    cell_state
                )
            }
        }
    }
    print!(
        "{}",
        termion::cursor::Goto(0, row_count as u16)
    );
    let _ = io::stdout().flush();
}

#[cfg(test)]
mod test {
    use std::{fmt, thread, time};
    use crate::grid::Grid;
    use super::Printer;

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum Test {
        A,
        B
    }

    impl std::fmt::Display for Test {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let str = match self {
                Test::A => {
                    "."
                },
                Test::B => {
                    "x"
                }
            };
            
            write!(f, "{}", str)
        }
    }

    #[test]
    fn can_print() {
        let mut state: Grid<Test> = Grid::new(Test::A);
        let time = time::Duration::from_millis(3000);
        let printer = Printer::new(state.clone());
        printer.start();
        thread::sleep(time);

        let col_count = state.grid.len();
        let row_count = state.grid[0].len();
        for col in 1..col_count+1 {
            for row in 1..row_count+1 {
                state.grid[col-1][row-1] = Test::B;
            }
        }

        let _ = printer.update(state.clone());
        thread::sleep(time);
    }
}