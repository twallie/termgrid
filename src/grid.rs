use std::fmt::Display;

#[derive(Clone)]
pub struct Grid<T> {
    // Grid represented as 2D Vector
    pub grid: Vec<Vec<T>>,
}

impl <T> Grid<T>
where T: Clone + Display {
    pub fn new(default: T) -> Self {
        let sizes = termion::terminal_size().unwrap();
        let column_count = sizes.0;
        let row_count = sizes.1;

        Self {
            grid: vec![vec![default; column_count as usize]; row_count as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt;

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
}
