use std;
use std::cmp::max;

use ncurses::{self, chtype};

use ui::window::{Window, WindowLike};

#[derive(Debug)]
pub enum InputErrorReason {
    DecodeError(std::str::Utf8Error),
}

#[derive(Debug)]
pub struct InputError {
    reason: InputErrorReason,
    // buffer so far for resumption
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "InputError({:?})", self.reason)
    }
}

impl std::error::Error for InputError {
    fn description(&self) -> &str {
        match self.reason {
            InputErrorReason::DecodeError(ref err) => err.description(),
        }
    }
}

impl From<std::str::Utf8Error> for InputError {
    fn from(err: std::str::Utf8Error) -> InputError {
        InputError {
            reason: InputErrorReason::DecodeError(err),
        }
    }
}

pub trait InputHandler {
    // Input functions can return an error if they are interrupted by
    // one of the handle* functions. TODO: add way to resume input
    // (parameter to function with buffer so far?)
    fn get_line(&self, window: &Window) -> Result<String, InputError> {
        let margins = window.margins();

        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
        ncurses::wmove(window.window(), margins.top, margins.left);
        window.refresh();

        let mut buf = vec![];

        let mut ch = ncurses::getch();
        let mut x = margins.left;
        while ch != ncurses::KEY_ENTER && ch != 13 {
            match ch {
                // Backspace
                127 => {
                    buf.pop();
                    x = max(x - 1, margins.left);
                    ncurses::wmove(window.window(), margins.top, x);
                    ncurses::waddch(window.window(), 32);
                }
                _ => {
                    ncurses::waddch(window.window(), ch as chtype);
                    buf.push(ch as u8);
                    x += 1;
                }
            }
            ncurses::wmove(window.window(), margins.top, x);
            window.refresh();
            ch = ncurses::getch();
        }

        // TODO: actually save and preserve
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        window.clear_line();
        Ok(try!(std::str::from_utf8(&buf).map(|s| s.to_owned())))
    }

    // fn prompt(&self, prompt: &str) {

    // }

    // fn handle_function(&self, key: chtype) -> Option<InputError>;
    // // TODO: add mouse struct as param
    // fn handle_mouse(&self) -> Option<InputError>;
}
