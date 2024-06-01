use crate::{errors::OutOfBoundsError, grid::{Cell, Grid}};

pub struct Printer {
    grid: Grid
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
           grid: Grid::new() 
        }
    }

    pub fn print(&self) -> () {
        for mut col_index in 0..64 {
            col_index = 63 - col_index;
            for row_index in 0..64 {
                let shown = match self.grid.get(row_index, col_index) {
                    Some(v) => {
                        match v {
                            Cell::Unmarked => '.',
                            Cell::Marked => 'O'
                        }
                    },
                    None => 'X',
                };
                print!("{shown}");
            }
            print!("\n");
        }
    }

    pub fn mark(&mut self, row_index: usize, col_index: usize) -> Result<(), OutOfBoundsError> {
        self.grid.mark(row_index, col_index) 
    }
}

#[cfg(test)]
mod test {
    use super::Printer;

    #[test]
    fn can_instantiate() {
        let _ = Printer::new();
        assert!(true);
    }

    #[test]
    fn print_doesnt_crash() {
        let p = Printer::new();
        p.print();
        assert!(true);
    }

    #[test]
    fn can_mark() {
        let mut p = Printer::new();
        let result = p.mark(0,0);
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }
}
