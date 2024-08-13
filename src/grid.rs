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