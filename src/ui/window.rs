use std::cmp::{max, min};
use std::str::{Utf8Error, from_utf8};

use ncurses::*;

pub enum ScrollMode {
    Wrap,
    Scroll,
}

pub struct Margins {
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

pub struct CursorPos {
    x: i32,
    y: i32,
}

pub struct Window {
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
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
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

    pub fn cursor_x(&self) -> i32 {
        self.cursor.x + self.margins.left
    }

    pub fn cursor_y(&self) -> i32 {
        self.cursor.y + self.margins.top
    }

    pub fn box_(&mut self, v: chtype, h: chtype) {
        self.margins.top = 1;
        self.margins.left = 1;
        self.margins.right = 1;
        self.margins.bottom = 1;
        box_(self.window, v, h);
    }

    pub fn refresh(&self) {
        wrefresh(self.window);
    }

    pub fn clear_line(&mut self) {
        for x in 0..self.width - self.margins.horizontal() {
            self.cursor.x = x;
            mvwaddch(self.window, self.cursor_y(), self.cursor_x(), 32);
        }
        self.cursor.x = 0;
        wmove(self.window, self.cursor_y(), self.cursor_x());
        self.refresh()
    }

    pub fn enable_scrolling(&mut self) {
        scrollok(self.window, true);
        self.scroll_mode = ScrollMode::Scroll;
    }

    // TODO: use pads
    pub fn print(&mut self, text: &str) {
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

    pub fn input(&mut self) -> Result<String, Utf8Error> {
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
                    self.cursor.x = max(self.cursor.x - 1, self.margins.left);
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
        from_utf8(&buf).map(|s| s.to_owned())
    }
}
