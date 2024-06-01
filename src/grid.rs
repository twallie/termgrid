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

    pub fn is_marked(&self, row_index: usize, col_index: usize) -> Option<bool> {
        if row_index >= Grid::MAX_SIZE || row_index < 0 || col_index >= Grid::MAX_SIZE || row_index < 0 {
            return None;
        }
        Some(matches!(self.cells[row_index][col_index], Cell::Marked))
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
        assert!(is_marked.unwrap() == false);

        tg.mark(row_index, col_index);

        is_marked = tg.is_marked(row_index, col_index);
        assert!(is_marked.unwrap() == true);
    }

    #[test]
    fn check_gives_none_when_out_of_bounds() {
        let tg = Grid::new();
        let result = tg.is_marked(Grid::MAX_SIZE, Grid::MAX_SIZE); 
        assert!(result.is_none());
    }
}
