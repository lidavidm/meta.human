use std::cell::RefCell;
use std::collections::HashMap;

use chrono;
use chrono::offset::TimeZone;

use room;

use ui;
use ui::input::InputHandler;
use ui::window::{Pad, ScrollingOutput, Window, WindowLike};

pub struct World {
    rooms: HashMap<String, room::Room>,
}

impl World {
    pub fn new(rooms: HashMap<String, room::Room>) -> World {
        World {
            rooms: rooms,
        }
    }

    pub fn add_room<S: Into<String>>(&mut self, id: S, room: room::Room) {
        self.rooms.insert(id.into(), room);
    }

    pub fn get_room(&self, id: &str) -> Option<&room::Room> {
        self.rooms.get(id)
    }
}

struct UILayout {
    border: Window,
    main: Window,
    output: RefCell<Pad>,
    character: Window,
    title: RefCell<Window>,
    input: Window,
}

impl UILayout {
    fn new() -> UILayout {
        let (width, height) = ui::term_size();
        let main_width = 2 * width / 3;
        let mut border_win = Window::new(0, 3, main_width, height - 6);
        border_win.box_(0, 0);
        border_win.refresh();
        let main_win = Window::new(1, 4, main_width - 2, height - 8);
        main_win.refresh();

        let output = Pad::new(100, 80);

        let mut char_info = Window::new(main_width, 0, width - main_width, height);
        char_info.box_(0, 0);
        char_info.refresh();

        let mut title_win = Window::new(0, 0, main_width, 3);
        title_win.box_(0, 0);
        title_win.refresh();

        let mut input_win = Window::new(0, height - 3, main_width, 3);
        input_win.box_(0, 0);
        input_win.refresh();

        UILayout {
            border: border_win,
            main: main_win,
            output: RefCell::new(output),
            character: char_info,
            title: RefCell::new(title_win),
            input: input_win,
        }
    }

    fn refresh(&self) {
        self.output.borrow_mut().render(&self.main);
    }

    fn set_title(&self, left: &str, right: &str) {
        let width = (ui::term_size().0 * 2 / 3) as usize - right.len() - 3;
        self.title.borrow_mut().print(0, &format!("{:<width$} {}", left, right, width=width));
    }

    fn display(&self, text: &str) {
        self.output.borrow_mut().append(text);
    }
}

impl InputHandler for UILayout {

}

pub struct Game {
    // player: ,
    pub world: World,
    ui: UILayout,
    pub room: String,
    pub time: chrono::DateTime<chrono::UTC>,
}

impl Game {
    pub fn new(world: World) -> Game {
        Game {
            ui: UILayout::new(),
            world: world,
            room: "NO_ROOM_SET".to_owned(),
            time: chrono::UTC.ymd(2048, 1, 2).and_hms(7, 7, 0),
        }
    }

    fn current_room(&self) -> &room::Room {
        self.world.get_room(&self.room).expect(&format!("Invalid room {}", self.room))
    }

    fn current_time(&self) -> String {
        format!("{}", self.time.format("%a %d %b %Y %H:%M"))
    }

    pub fn enter_room(&mut self, room: &str) -> Option<()> {
        if !self.world.rooms.contains_key(room) {
            None
        }
        else {
            self.room = room.into();
            let room = self.current_room();
            self.ui.display(&room.description);
            self.ui.set_title(&room.name, &self.current_time());
            Some(())
        }
    }

    pub fn main(&mut self) {
        self.ui.refresh();
        loop {
            let input = self.ui.get_line(&self.ui.input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            let parts: Option<(&&str, &[&str])> = parts.split_first();
            if parts.is_none() {
                continue;
            }
            let (command, args) = parts.unwrap();

            match *command {
                "exit" => break,
                "describe" => self.ui.display(&self.current_room().description),
                "go" => {
                    if args.is_empty() {
                        self.ui.display("Go where?");
                    }
                    else {
                        let target = args.join(" ");
                        let result = { self.current_room().find_door(&target).map(|x| x.to_owned()) };
                        if let Some(room_name) = result {
                            self.enter_room(&room_name).unwrap();
                        }
                        else {
                            self.ui.display(&format!("Can't go to {}", target));
                        }
                    }
                },
                _ => {
                    self.ui.display(input.as_ref());
                },
            }

            self.ui.refresh();
        }
    }
}
