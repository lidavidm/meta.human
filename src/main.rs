#![feature(braced_empty_structs)]
extern crate chrono;
extern crate ncurses;
extern crate yaml_rust;

#[macro_use]
mod assets;
mod player;
mod room;
mod state;
mod ui;

use std::io::Read;
use std::fs::File;
use std::path::Path;

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

    let mut room_contents = String::new();
    let mut file = File::open(&Path::new("rooms.yaml")).unwrap();
    file.read_to_string(&mut room_contents).unwrap();

    let docs = yaml_rust::YamlLoader::load_from_str(&room_contents).unwrap();
    let rooms = room::load_rooms(docs).unwrap();

    let mut game = state::Game::new(state::World::new(rooms));

    game.enter_room("border_office_1").unwrap();
    game.main();

    // output.append("ACT I—HELLO WORLD");
    // output.append("=================");
    // output.append("");
    // output.render(&main_win);


    // title_win.print(0, &format!("Location: {:20} 17:07:17 MON 25 MAR 2048", cur_room.name));

    endwin();
}
