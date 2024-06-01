use crate::grid::Grid;

pub struct Printer {
    grid: Grid
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
           grid: Grid::new() 
        }
    }
}

#[cfg(test)]
mod test {
    use super::Printer;

    #[test]
    fn can_instantiate() {
        let _ = Printer::new();
        assert!(true);
    }
}
