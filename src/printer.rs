use crate::grid::VectorGrid;

pub fn clear_screen() {
    print!("{}", termion::clear::All)
}

pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn goto_end() {
    let size = termion::terminal_size().unwrap();
    print!("{}\n", termion::cursor::Goto(size.0, size.1))
}

pub fn print_in_place<T>(grid: &VectorGrid<T>)
where
    T: Clone + Eq + PartialEq,
{
    for data_row_index in 0..grid.get_rows() {
        for data_column_index in 0..grid.get_columns() {
            let num_rows = grid.get_rows();
            let screen_row_index = num_rows - data_row_index;
            let screen_column_index = 1 + data_column_index;

            // maybe handle this in the future but honestly i'm okay with it
            // panicking here since this should always work since we get the size of the
            // underlying structure... but it feels wrong...
            let element = grid.get_element(data_column_index, data_row_index).unwrap();
            let mut print_str = " ";
            if *element == *grid.get_filled() {
                print_str = "#"
            }
            print!(
                "{}{}",
                termion::cursor::Goto(screen_column_index as u16, screen_row_index as u16),
                print_str
            )
        }
    }
}
