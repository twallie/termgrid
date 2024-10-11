use crate::{
    grid::{OutOfBoundsError, VectorGrid},
    printer::{clear_screen, goto_end, print_in_place},
};

pub struct TermGrid<T> {
    grid: VectorGrid<T>,
}

impl<T> TermGrid<T>
where
    T: Clone + Eq + PartialEq,
{
    pub fn new(filled: &T, empty: &T) -> TermGrid<T> {
        // TODO: properly handle this
        let size = termion::terminal_size().unwrap();
        let rows: usize = size.1.into();
        let columns: usize = size.0.into();

        let grid: VectorGrid<T> = VectorGrid::new(&filled, &empty, rows, columns);

        TermGrid { grid }
    }

    pub fn set(&mut self, column: usize, row: usize) -> Result<(), OutOfBoundsError> {
        self.grid.set_element(column, row)
    }

    pub fn unset(&mut self, column: usize, row: usize) -> Result<(), OutOfBoundsError> {
        self.grid.unset_element(column, row)
    }

    pub fn start(&self) {
        clear_screen();
    }

    pub fn update(&self) {
        print_in_place(&self.grid)
    }

    pub fn end(&self) {
        goto_end();
    }

    pub fn get(&self, column: usize, row: usize) -> Result<&T, OutOfBoundsError> {
        self.grid.get_element(column, row)
    }
}

#[cfg(test)]
mod tests {
    use crate::controller::TermGrid;

    #[derive(Clone, Eq, PartialEq)]
    enum TestEnum {
        Filled,
        Empty,
    }

    #[test]
    fn it_works() {
        let mut termgrid = TermGrid::new(&TestEnum::Filled, &TestEnum::Empty);
        termgrid.set(0, 0).unwrap();
        termgrid.set(1, 1).unwrap();
        termgrid.start();
        termgrid.update();
        termgrid.end();

        assert_eq!(4, 4);
    }
}
