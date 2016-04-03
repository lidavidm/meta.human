#![feature(braced_empty_structs)]
extern crate ncurses;

mod player;
mod room;
mod state;
mod ui;

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

    let mut game = state::Game::new();

    let room = room::Room {
        name: "Border Office".to_owned(),
        description: "Florescent light glares off the metal detector ahead.".to_owned(),
    };
    game.world.add_room("border_office_1", room);
    game.enter_room("border_office_1");
    game.main();

    // output.append("ACT I—HELLO WORLD");
    // output.append("=================");
    // output.append("");
    // output.render(&main_win);


    // title_win.print(0, &format!("Location: {:20} 17:07:17 MON 25 MAR 2048", cur_room.name));

    endwin();
}
