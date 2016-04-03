use std::cmp::{max, min};
use std::str::{Utf8Error, from_utf8};

use ncurses::*;

#[derive(Clone,Copy,Debug)]
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

pub trait WindowLike {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn margins(&self) -> &Margins;

    fn cursor_pos(&self) -> CursorPos {
        let mut x = 0;
        let mut y = 0;
        getyx(self.window(), &mut y, &mut x);

        CursorPos {
            x: x,
            y: y,
        }
    }

    fn window(&self) -> WINDOW;
    fn refresh(&self);

    fn clear_line(&self) {
        let cursor = self.cursor_pos();
        let margins = self.margins();
        for x in margins.left..self.width() - margins.horizontal() {
            mvwaddch(self.window(), cursor.y, x, 32);
        }
        wmove(self.window(), cursor.y, margins.left);
        self.refresh()
    }

    // TODO: print, input should be moved to a different trait
    fn print(&mut self, row: i32, text: &str) {
        let margins = self.margins();
        wmove(self.window(), row + margins.top, margins.left);
        self.clear_line();

        wprintw(self.window(), text);

        self.refresh()
    }

    // TODO: there should be a global input handler that can watch for things like mouse events
    fn input(&mut self) -> Result<String, Utf8Error> {
        let margins = self.margins();

        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        wmove(self.window(), margins.top, margins.left);
        self.refresh();

        let mut buf = vec![];

        let mut ch = getch();
        let mut x = margins.left;
        while ch != KEY_ENTER && ch != 13 {
            match ch {
                // Backspace
                127 => {
                    buf.pop();
                    x = max(x - 1, margins.left);
                    wmove(self.window(), margins.top, x);
                    waddch(self.window(), 32);
                }
                _ => {
                    waddch(self.window(), ch as chtype);
                    buf.push(ch as u8);
                    x += 1;
                }
            }
            wmove(self.window(), margins.top, x);
            self.refresh();
            ch = getch();
        }

        // TODO: actually save and preserve
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        self.clear_line();
        from_utf8(&buf).map(|s| s.to_owned())
    }
}

pub trait ScrollingOutput: WindowLike {
    fn current_row(&self) -> i32;
    fn advance_row(&mut self);
    fn append(&mut self, text: &str) {
        {
            let margins = self.margins();

            wmove(self.window(), self.current_row() + margins.top, margins.left);
            self.clear_line();

            wprintw(self.window(), text);
        }

        self.advance_row();
        self.refresh();
    }
}

pub struct Window {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    margins: Margins,
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
            window: newwin(height, width, y, x),
        }
    }

    pub fn box_(&mut self, v: chtype, h: chtype) {
        self.margins.top = 1;
        self.margins.left = 1;
        self.margins.right = 1;
        self.margins.bottom = 1;
        box_(self.window, v, h);
    }
}

impl WindowLike for Window {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn margins(&self) -> &Margins {
        &self.margins
    }

    fn window(&self) -> WINDOW {
        self.window
    }

    fn refresh(&self) {
        wrefresh(self.window);
    }
}

pub struct Pad {
    window: WINDOW,
    lines: i32,
    columns: i32,
    margins: Margins,
    row: i32,
}

impl Pad {
    pub fn new(lines: i32, columns: i32) -> Pad {
        Pad {
            window: newpad(lines, columns),
            lines: lines,
            columns: columns,
            margins: Margins {
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
            },
            row: 0,
        }
    }

    pub fn render(&self, window: &WindowLike) {
        let margins = window.margins();
        let scroll = max(0, self.row - window.height());
        prefresh(self.window(),
                 margins.top + scroll, margins.left,
                 self.margins.top + window.y(),
                 self.margins.left + window.x(),
                 window.y() + window.height() - margins.vertical() - 1,
                 window.x() + window.width() - margins.horizontal() - 1);
    }
}

impl WindowLike for Pad {
    // TODO:
    fn x(&self) -> i32 {
        0
    }

    fn y(&self) -> i32 {
        0
    }

    fn width(&self) -> i32 {
        self.columns
    }

    fn height(&self) -> i32 {
        self.lines
    }

    fn margins(&self) -> &Margins {
        &self.margins
    }

    fn window(&self) -> WINDOW {
        self.window
    }

    fn refresh(&self) {

    }
}

impl ScrollingOutput for Pad {
    fn current_row(&self) -> i32 {
        self.row
    }

    fn advance_row(&mut self) {
        if self.row == self.height() - (self.margins().vertical() + 1) {
            wscrl(self.window(), 1);
        }
        self.row = min(self.row + 1, self.height() - self.margins().vertical())
    }
}
