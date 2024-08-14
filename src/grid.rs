use std::{cell::Cell, fmt::Display, vec};

#[derive(Clone)]
pub struct Grid<T> {
    // Grid represented as 2D Vector
    vector: Vec<Vec<T>>,
    height: usize,
    length: usize
}

impl <CellType> Grid<CellType>
where CellType: Clone + Display {
    pub fn new(default: CellType) -> Self {
        let sizes = termion::terminal_size().unwrap();
        let column_count = sizes.0;
        let row_count = sizes.1;
        
        let vector = vec![vec![default; column_count as usize]; row_count as usize];
        let height = vector.len();
        let length = vector[0].len();

        Self {
            vector,
            height,
            length, 
        }
    }

    /// Sets the given grid position to the specified value.
    /// 
    /// Remember, grid position looks like:
    /// ```
    ///  y
    /// 3|
    /// 2|
    /// 1|
    /// 0|
    ///  +-------x
    ///   0 1 2 3
    /// ```
    pub fn set(&mut self, x: usize, y: usize, value: CellType) {
        self.vector[y][x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> &CellType {
        &self.vector[y][x]
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn length(&self) -> usize {
        self.length
    }
}