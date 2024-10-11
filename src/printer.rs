use crate::grid::VectorGrid;
fn clear_screen() {
    print!("{}", termion::clear::All)
}

fn goto_end() {
    let size = termion::terminal_size().unwrap();
    print!("{}\n", termion::cursor::Goto(size.0, size.1))
}

fn print_in_place<T>(grid: VectorGrid<T>)
where
    T: Clone + Eq + PartialEq,
{
    for data_row_index in 0..grid.get_rows() {
        for data_column_index in 0..grid.get_columns() {
            let num_rows = grid.get_rows();
            let screen_row_index = num_rows - data_row_index;
            let screen_column_index = 1 + data_column_index;

            // TODO: actually handle this
            let element = grid.get_element(data_column_index, data_row_index).unwrap();
            let mut print_str = ".";
            if *element == *grid.get_filled() {
                print_str = "*"
            }
            print!(
                "{}{}",
                termion::cursor::Goto(screen_column_index as u16, screen_row_index as u16),
                print_str
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grid::VectorGrid,
        printer::{goto_end, print_in_place},
    };

    #[derive(Clone, Eq, PartialEq)]
    enum TestEnum {
        Filled,
        Empty,
    }

    #[test]
    fn it_works() {
        let mut grid = VectorGrid::new(TestEnum::Filled, TestEnum::Empty, 5, 5);
        grid.set_element(0, 0, TestEnum::Filled).unwrap();
        grid.set_element(1, 1, TestEnum::Filled).unwrap();
        print!("{}", termion::clear::All);
        print_in_place(grid);
        goto_end();

        assert_eq!(4, 4);
    }
}
