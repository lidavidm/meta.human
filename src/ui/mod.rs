use ncurses;

pub mod window;

pub fn term_size() -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    ncurses::getmaxyx(ncurses::stdscr, &mut y, &mut x);

    (x, y)
}
