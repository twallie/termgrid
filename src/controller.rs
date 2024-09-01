use crate::grid::{self, CellError, Grid, TerminalSizeError};

pub struct Controller<T> {
    grid: Grid<T>,
}

impl<T> Controller<T>
where
    T: Clone,
{
    pub fn new(default: &T) -> Result<Controller<T>, TerminalSizeError> {
        let grid = match Grid::new(default) {
            Ok(v) => v,
            Err(v) => return Err(v),
        };
        Ok(Controller { grid })
    }

    pub fn set(&mut self, x: &usize, y: &usize, value: T) -> Result<(), CellError> {
        match self.grid.set_cell(x, y, value) {
            Ok(_) => Ok(()),
            Err(v) => return Err(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::Controller;

    #[derive(Clone, PartialEq)]
    enum TestStates {
        Filled,
        Empty,
    }

    impl fmt::Display for TestStates {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::Filled => write!(f, "*"),
                Self::Empty => write!(f, " "),
            }
        }
    }

    #[test]
    fn can_instantiate() {
        let _ = Controller::new(&TestStates::Empty).unwrap();
    }

    #[test]
    fn can_set() {
        let mut controller = Controller::new(&TestStates::Empty).unwrap();
        controller.set(&0, &0, TestStates::Filled).unwrap();
    }
}
