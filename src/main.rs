extern crate ncurses;

use std::str;

use ncurses::*;

fn term_size() -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    getmaxyx(stdscr, &mut y, &mut x);

    (x, y)
}

enum ScrollMode {
    Wrap,
    Scroll,
}

struct Margins {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Margins {
    fn horizontal(&self) -> i32 {
        self.left + self.right
    }

    fn vertical(&self) -> i32 {
        self.top + self.bottom
    }
}

struct CursorPos {
    x: i32,
    y: i32,
}

struct Window {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    margins: Margins,
    cursor: CursorPos,
    scroll_mode: ScrollMode,
    window: WINDOW,
}

impl Window {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        Window {
            x: x,
            y: y,
            width: width,
            height: height,
            margins: Margins {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
            },
            cursor: CursorPos {
                x: 0,
                y: 0,
            },
            scroll_mode: ScrollMode::Wrap,
            window: newwin(height, width, y, x),
        }
    }

    fn cursor_x(&self) -> i32 {
        self.cursor.x + self.margins.left
    }

    fn cursor_y(&self) -> i32 {
        self.cursor.y + self.margins.top
    }

    fn box_(&mut self, v: chtype, h: chtype) {
        self.margins.top = 1;
        self.margins.left = 1;
        self.margins.right = 1;
        self.margins.bottom = 1;
        box_(self.window, v, h);
    }

    fn refresh(&self) {
        wrefresh(self.window);
    }

    fn clear_line(&mut self) {
        for x in 0..self.width - self.margins.horizontal() {
            self.cursor.x = x;
            mvwaddch(self.window, self.cursor_y(), self.cursor_x(), 32);
        }
        self.cursor.x = 0;
        wmove(self.window, self.cursor_y(), self.cursor_x());
        self.refresh()
    }

    fn enable_scrolling(&mut self) {
        scrollok(self.window, true);
        self.scroll_mode = ScrollMode::Scroll;
    }

    fn print(&mut self, text: &str) {
        wmove(self.window, self.cursor_y(), self.cursor_x());
        self.clear_line();

        wprintw(self.window, text);

        self.cursor.y = match self.scroll_mode {
            ScrollMode::Wrap => {
                // TODO: effective/inner height function
                if self.cursor.y + 1 >= self.height - self.margins.vertical() {
                    0
                }
                else {
                    self.cursor.y + 1
                }
            }
            ScrollMode::Scroll => {
                if self.cursor.y >= self.height - self.margins.vertical() {
                    wscrl(self.window, 1);
                }
                self.cursor.y + 1
            },
        };
        self.refresh()
    }

    fn input(&mut self) -> Result<String, std::str::Utf8Error> {
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        wmove(self.window, self.cursor_y(), self.cursor_x());
        self.refresh();

        let mut buf = vec![];

        let mut ch = getch();
        while ch != KEY_ENTER && ch != 13 {
            match ch {
                // Backspace
                127 => {
                    buf.pop();
                    self.cursor.x = std::cmp::max(self.cursor.x - 1, self.margins.left);
                    wmove(self.window, self.cursor_y(), self.cursor_x());
                    waddch(self.window, 32);
                }
                _ => {
                    waddch(self.window, ch as chtype);
                    buf.push(ch as u8);
                    self.cursor.x += 1;
                }
            }
            wmove(self.window, self.cursor_y(), self.cursor_x());
            self.refresh();
            ch = getch();
        }

        // TODO: actually save and preserve
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        self.clear_line();
        str::from_utf8(&buf).map(|s| s.to_owned())
    }
}

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

    let (width, height) = term_size();
    let main_width = 2 * width / 3;
    let mut border_win = Window::new(0, 3, main_width, height - 6);
    border_win.box_(0, 0);
    border_win.refresh();
    let mut main_win = Window::new(1, 4, main_width - 2, height - 8);
    main_win.enable_scrolling();
    main_win.refresh();

    let mut char_info = Window::new(main_width, 0, width - main_width, height);
    char_info.box_(0, 0);
    char_info.refresh();

    let mut title_win = Window::new(0, 0, main_width, 3);
    title_win.box_(0, 0);
    title_win.refresh();

    let mut input_win = Window::new(0, height - 3, main_width, 3);
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
