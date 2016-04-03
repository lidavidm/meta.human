extern crate ncurses;

mod ui;
mod state;
mod room;
mod player;

use ncurses::*;

fn main() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");
    initscr();
    raw();
    nonl();
    keypad(stdscr, true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    refresh();

    let (width, height) = ui::term_size();
    let main_width = 2 * width / 3;
    let mut border_win = ui::window::Window::new(0, 3, main_width, height - 6);
    border_win.box_(0, 0);
    border_win.refresh();
    let mut main_win = ui::window::Window::new(1, 4, main_width - 2, height - 8);
    main_win.enable_scrolling();
    main_win.refresh();

    let mut char_info = ui::window::Window::new(main_width, 0, width - main_width, height);
    char_info.box_(0, 0);
    char_info.refresh();

    let mut title_win = ui::window::Window::new(0, 0, main_width, 3);
    title_win.box_(0, 0);
    title_win.refresh();

    let mut input_win = ui::window::Window::new(0, height - 3, main_width, 3);
    input_win.box_(0, 0);
    input_win.refresh();

    main_win.print("Hello, 世界!");

    loop {
        let input = input_win.input().unwrap();
        main_win.print(&input);

        if input == "exit" {
            break;
        }
    }

    endwin();
}
