use crate::errors::OutOfBoundsError;

#[derive(Clone)]
pub struct VectorGrid<T> {
    data: Vec<Vec<T>>,
    empty: T,
    filled: T,
}

impl<T> VectorGrid<T>
where
    T: Clone,
{
    pub fn new(filled: &T, empty: &T, rows: usize, columns: usize) -> VectorGrid<T> {
        VectorGrid {
            data: vec![vec![empty.clone(); columns]; rows],
            empty: empty.clone(),
            filled: filled.clone(),
        }
    }

    pub fn get_columns(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_rows(&self) -> usize {
        self.data.len()
    }

    pub fn unset_element(
        &mut self,
        column_index: usize,
        row_index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let row = match self.data.get_mut(row_index) {
            Some(v) => v,
            None => return Err(OutOfBoundsError),
        };

        match row.get_mut(column_index) {
            Some(v) => *v = self.empty.clone(),
            None => return Err(OutOfBoundsError),
        };

        Ok(())
    }

    pub fn set_element(
        &mut self,
        column_index: usize,
        row_index: usize,
    ) -> Result<(), OutOfBoundsError> {
        let row = match self.data.get_mut(row_index) {
            Some(v) => v,
            None => return Err(OutOfBoundsError),
        };

        match row.get_mut(column_index) {
            Some(v) => *v = self.filled.clone(),
            None => return Err(OutOfBoundsError),
        };

        Ok(())
    }

    pub fn get_filled(&self) -> &T {
        &self.filled
    }

    pub fn get_empty(&self) -> &T {
        &self.filled
    }

    pub fn get_element(
        &self,
        column_index: usize,
        row_index: usize,
    ) -> Result<&T, OutOfBoundsError> {
        let row = match self.data.get(row_index) {
            Some(v) => v,
            None => return Err(OutOfBoundsError),
        };

        match row.get(column_index) {
            Some(v) => return Ok(v),
            None => return Err(OutOfBoundsError),
        };
    }
}
