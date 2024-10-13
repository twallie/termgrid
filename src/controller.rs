use crate::{
    errors::{OutOfBoundsError, TerminalSizeError},
    grid::VectorGrid,
    printer::{clear_screen, goto_end, hide_cursor, print_in_place, show_cursor},
};

#[derive(Clone)]
pub struct TermGrid<T> {
    grid: VectorGrid<T>,
}

/// Returns a tuple with the form (columns, rows)
fn get_terminal_size() -> Result<(usize, usize), TerminalSizeError> {
    match termion::terminal_size() {
        Ok(v) => Ok((v.0.into(), v.1.into())),
        Err(_) => Err(TerminalSizeError),
    }
}

impl<T> TermGrid<T>
where
    T: Clone + Eq + PartialEq,
{
    pub fn new(filled: &T, empty: &T) -> Result<TermGrid<T>, TerminalSizeError> {
        let size = match get_terminal_size() {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        let rows: usize = size.1.into();
        let columns: usize = size.0.into();

        let grid: VectorGrid<T> = VectorGrid::new(&filled, &empty, rows, columns);

        Ok(TermGrid { grid })
    }

    pub fn num_rows(&self) -> usize {
        self.grid.get_rows()
    }

    pub fn num_columns(&self) -> usize {
        self.grid.get_columns()
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        self.grid.set_element(x, y)
    }

    pub fn unset(&mut self, x: usize, y: usize) -> Result<(), OutOfBoundsError> {
        self.grid.unset_element(x, y)
    }

    pub fn start(&self) {
        clear_screen();
        hide_cursor();
    }

    pub fn update(&self) {
        print_in_place(&self.grid)
    }

    pub fn end(&self) {
        goto_end();
        show_cursor();
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&T, OutOfBoundsError> {
        self.grid.get_element(x, y)
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
        let mut termgrid = TermGrid::new(&TestEnum::Filled, &TestEnum::Empty).unwrap();
        termgrid.set(0, 0).unwrap();
        termgrid.set(1, 1).unwrap();
        termgrid.start();
        termgrid.update();
        termgrid.end();

        assert_eq!(4, 4);
    }
}
