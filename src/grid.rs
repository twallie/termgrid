use termion::terminal_size;

pub struct TerminalGrid<T> {
    vec: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct TerminalSizeError;

#[derive(Debug)]
pub enum GetError {
    ColumnOutOfBounds,
    RowOutOfBounds
}

impl<T> TerminalGrid<T>
where
    T: Clone,
{
    pub fn new(default: &T) -> Result<TerminalGrid<T>, TerminalSizeError> {
        let size = match terminal_size() {
            Ok(v) => v,
            Err(_) => return Err(TerminalSizeError),
        };
        let column_count = size.0 as usize;
        let row_count = size.1 as usize;

        Ok(TerminalGrid {
            vec: vec![vec![default.clone(); row_count]; column_count],
        })
    }

    pub fn get(&self, x: &usize, y: &usize) -> Result<&T, GetError> {
        let row = match &self.vec.get(*y) {
            &Some(v) => v,
            &None => return Err(GetError::RowOutOfBounds)
        };

        let value = match row.get(*x) {
            Some(v) => v,
            None => return Err(GetError::ColumnOutOfBounds)
        };

        Ok(value)
    }

    pub fn set(&mut self, x: &usize, y: &usize, value: T) -> Result<(), GetError> {
        let row = match self.vec.get_mut(*y) {
            Some(v) => v,
            None => return Err(GetError::RowOutOfBounds)
        };

        let cell = match row.get_mut(*x) {
            Some(v) => v,
            None => return Err(GetError::ColumnOutOfBounds)
        };

        *cell = value;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq)]
    enum TestStates {
        Filled,
        Empty
    }

    #[test]
    fn new() {
        match TerminalGrid::new(&TestStates::Empty) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        };
    }

    #[test]
    fn get_without_setting() {
        let grid = TerminalGrid::new(&TestStates::Empty).unwrap();
        match grid.get(&0, &0) {
            Ok(v) => assert!(v == &TestStates::Empty),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn set_and_get() {
        let mut grid = TerminalGrid::new(&TestStates::Empty).unwrap();
        match grid.get(&0, &0) {
            Ok(v) => assert!(v == &TestStates::Empty),
            Err(_) => assert!(false)
        }

        match grid.set(&0, &0, TestStates::Filled) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        match grid.get(&0, &0) {
            Ok(v) => assert!(v == &TestStates::Filled),
            Err(_) => assert!(false)
        }
    }
}