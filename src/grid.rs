const MAX_GRID_SIZE: usize = 64;

/// A representation of a Cell in the [Grid].
/// Each cell can have two states:
/// [Cell::Unmarked] - cell is set to OFF
/// [Cell::Marked] - cell is set to ON
#[derive(Copy, Clone)]
pub enum Cell {
    Unmarked,
    Marked
}

/// An error indicating the action could not be completed due to
/// the fact that it is out of the bounds of the grid.
pub struct OutOfBoundsError;

/// A representation of the underlying Grid shown to the user in the terminal.
/// The grid is a 64x64 2-D array, with each element being a [Cell].
pub struct Grid {
    cells: [[Cell; MAX_GRID_SIZE]; MAX_GRID_SIZE]
}

impl Grid {
    /// The max size of the grid.
    /// This is the maximum number of slots both in a row and column.
    pub const MAX_SIZE: usize = MAX_GRID_SIZE;

    /// Creates a new blank Grid object, with all elements set to [Cell::Unmarked]
    pub fn new() -> Grid {
        Grid {
            cells: [[Cell::Unmarked; Grid::MAX_SIZE]; Grid::MAX_SIZE]
        }
    }

    /// Attempts to get the value of the specified cell at the given `row_index` and `col_index`.
    /// Returns an [Option]. If successful, the option will yield an [Cell]. If unsuccessful,
    /// the option will yield [None]
    pub fn get(&self, row_index: usize, col_index: usize) -> Option<Cell> {
        if !Grid::in_bounds(row_index, col_index) {
            return None;
        }
        Some(self.cells[row_index][col_index])
    }

    /// Attempts to mark the cell at (`row_index`,`col_index`) as [Cell::Marked].
    /// Returns a [Result]. If successful (or cell is already marked), nothing is given.
    /// If the given indices are out of bounds, a [OutOfBoundsError] will be given.
    pub fn mark(&mut self, row_index: usize, col_index: usize) -> Result<(), OutOfBoundsError> {
        if !Grid::in_bounds(row_index, col_index) {
            return Err(OutOfBoundsError);
        }
        self.cells[row_index][col_index] = Cell::Marked;
        Ok(())
    }

    fn in_bounds(row_index: usize, col_index: usize) -> bool {
        row_index < Grid::MAX_SIZE && col_index < Grid::MAX_SIZE 
    }
}

#[cfg(test)]
mod test {
    use super::{Cell, Grid, OutOfBoundsError};

    #[test] 
    fn can_instantiate() {
        let _tg = Grid::new();
        assert!(true);
    }

    #[test]
    fn mark_and_get_cell() {
        let mut tg = Grid::new();

        let row_index = 0;
        let col_index = 0;
        let _unmarked = tg.get(row_index, col_index).unwrap();
        assert!(matches!(Cell::Unmarked, _unmarked));

        let _ = tg.mark(row_index, col_index);

        let _marked = tg.get(row_index, col_index).unwrap();
        assert!(matches!(Cell::Marked, _marked));
    }

    #[test]
    fn get_gives_none_when_out_of_bounds() {
        let tg = Grid::new();
        let result = tg.get(Grid::MAX_SIZE, Grid::MAX_SIZE); 
        assert!(result.is_none());
    }

    #[test]
    fn mark_gives_err_when_out_of_bounds() {
        let oob_row = 64;
        let oob_col = 64;
        let mut tg = Grid::new();
        match tg.mark(oob_row, oob_col) {
            Ok(_) => assert!(false),
            Err(v) => assert!(matches!(v, OutOfBoundsError)),
        }
    }
}
