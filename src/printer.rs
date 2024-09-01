
use std::fmt::Display;

use crate::grid::Grid;

#[derive(Debug)]
pub struct PrintError;

pub fn print_grid<T>(grid: &Grid<T>) -> Result<(), PrintError>
where T: Clone + Display {
    clear_screen();

    for term_y in 1..grid.get_height()+1 {
        for term_x in 1..grid.get_length()+1 {
            let x = term_x - 1;
            let y = grid.get_height() - term_y;

            let cell_value = match grid.get_cell(&x, &y) {
                Ok(v) => v,
                Err(_) => { 
                    println!("ERROR!");
                    return Err(PrintError);
                }
            };
            set_terminal_cell(&term_x, &term_y, cell_value);
        }
    }
    finish_print(grid.get_height());
    Ok(())
}

fn set_terminal_cell<T>(x: &usize, y: &usize, cell_value: &T)
where T: Display {
    print!("{}{}", termion::cursor::Goto(*x as u16, *y as u16), cell_value);
}

fn finish_print(term_height: &usize) {
    print!("{}\n", termion::cursor::Goto(0, *term_height as u16));
}

fn clear_screen() {
    print!("{}", termion::clear::All);
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;
    use std::fmt;

    use super::print_grid;

    #[derive(Clone, PartialEq)]
    enum TestStates {
        Filled,
        Empty
    }

    impl fmt::Display for TestStates {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::Filled => write!(f, "*"),
                Self::Empty => write!(f, " ")
            }
        }
    }

    #[test]
    fn can_print() {
        let mut grid = Grid::new(&TestStates::Empty).unwrap();
        grid.set_cell(&0, &0, TestStates::Filled).unwrap();
        grid.set_cell(&0, &(grid.get_height() - 1), TestStates::Filled).unwrap();
        grid.set_cell(&(grid.get_length() - 1), &0, TestStates::Filled).unwrap();
        grid.set_cell(&(grid.get_length() - 1), &(grid.get_height() - 1), TestStates::Filled).unwrap();
        print_grid(&grid).unwrap();
    }
}