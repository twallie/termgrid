use termion::terminal_size;

pub struct TerminalGrid<T> {
    vec: Vec<Vec<T>>,
}

pub enum TerminalGridError {
    SizeError,
}

impl<T> TerminalGrid<T>
where
    T: Clone,
{
    pub fn new(&self, default: T) -> Result<TerminalGrid<T>, TerminalGridError> {
        let size = match terminal_size() {
            Ok(v) => v,
            Err(_) => return Err(TerminalGridError::SizeError),
        };
        let column_count = size.0 as usize;
        let row_count = size.1 as usize;

        Ok(TerminalGrid {
            vec: vec![vec![default; row_count]; column_count],
        })
    }
}
