// Dirty little ansi hack to place the cursor where we want it
pub fn cursor_at_position(row: u8, column: u8) {
    print!("{}[{};{}H", 27 as char, row, column);
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn hide_cursor() {
    print!("{}[?25l", 27 as char);
}

pub fn show_cursor() {
    print!("{}[?25h", 27 as char);
}
