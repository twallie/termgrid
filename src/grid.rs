#[derive(Copy, Clone)]
enum Cell {
    Unmarked,
    Marked
}

struct TermGrid {
    cells: [[Cell; 64]; 64]
}

impl TermGrid {
    pub fn new() -> TermGrid {
        TermGrid {
            cells: [[Cell::Unmarked; 64]; 64]
        }
    }
}

#[cfg(test)]
mod test {
    use super::TermGrid;

    #[test] 
    fn can_instantiate() {
        let _tg = TermGrid::new();
        assert!(true);
    }
}
