use std::fmt::Display;

struct Grid<T> {
    // Grid represented as 2D Vector
    grid: Vec<Vec<T>>,
}

impl <T> Grid<T>
where T: Clone + Display {
    fn new(default: T) -> Self {
        Self {
            grid: vec![vec![default; 5]]
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;
    use std::{io::{self, Write}, time, thread};

    use super::Grid;

    #[test]
    fn instantiate() {
        #[derive(Clone)]
        enum Test {
            A,
        }

        impl std::fmt::Display for Test {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let str = match self {
                    Test::A => {
                        "A"
                    },
                };
                
                write!(f, "{}", str)
            }
        }

        let _ = Grid::new(Test::A);
    }

    #[test]
    fn test() {
        let sizes = termion::terminal_size().unwrap();
        let column_count = sizes.0;
        let row_count = sizes.1;

        print!("{}", termion::clear::All);

        for col in 1..column_count+1 {
            for row in 1..row_count+1 {
                print!(
                    "{}.",
                    termion::cursor::Goto(col, row)
                )
            }
        }

        io::stdout().flush().unwrap();

        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);
    }
}
