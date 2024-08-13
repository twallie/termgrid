use std::io::{stdout, Write};

fn reset_cursor_internal() {
    let size = termion::terminal_size().unwrap();
    print!("{}", termion::cursor::Goto(0, size.1 + 1));
    stdout().flush().unwrap();
}

fn show_cursor() {
    print!("{}", termion::cursor::Show);
}

pub fn reset_cursor() {
    reset_cursor_internal();
    show_cursor();
    print!("\n");
}