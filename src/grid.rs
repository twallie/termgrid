#[derive(Copy, Clone)]
enum Cell {
    Unmarked,
    Marked
}

const MAX_GRID_SIZE: usize = 64;

pub struct Grid {
    cells: [[Cell; MAX_GRID_SIZE]; MAX_GRID_SIZE]
}

impl Grid {
    pub const MAX_SIZE: usize = MAX_GRID_SIZE;

    pub fn new() -> Grid {
        Grid {
            cells: [[Cell::Unmarked; Grid::MAX_SIZE]; Grid::MAX_SIZE]
        }
    }

    pub fn is_marked(&self, row_index: usize, col_index: usize) -> bool {
        matches!(self.cells[row_index][col_index], Cell::Marked)
    }

    pub fn mark(&mut self, row_index: usize, col_index: usize) -> () {
        self.cells[row_index][col_index] = Cell::Marked;
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

    #[test] 
    fn can_instantiate() {
        let _tg = Grid::new();
        assert!(true);
    }

    #[test]
    fn can_mark_and_check_cell() {
        let mut tg = Grid::new();

        let row_index = 0;
        let col_index = 0;
        let mut is_marked = tg.is_marked(row_index, col_index);
        assert!(!is_marked);

        tg.mark(row_index, col_index);

        is_marked = tg.is_marked(row_index, col_index);
        assert!(is_marked);
    }

    #[test]
    #[should_panic]
    fn panic_when_checking_out_of_bounds() {
        let tg = Grid::new();
        let _result = tg.is_marked(Grid::MAX_SIZE, Grid::MAX_SIZE); 
    }
}
