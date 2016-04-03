use std::cell::RefCell;
use std::collections::HashMap;

use room;

use ui;
use ui::input::InputHandler;
use ui::window::{Pad, ScrollingOutput, Window, WindowLike};

pub struct World {
    rooms: HashMap<String, room::Room>,
}

impl World {
    pub fn new() -> World {
        World {
            rooms: HashMap::new(),
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
    title: Window,
    input: Window,
}

impl UILayout {
    fn new() -> UILayout {
        let (width, height) = ui::term_size();
        let main_width = 2 * width / 3;
        let mut border_win = Window::new(0, 3, main_width, height - 6);
        border_win.box_(0, 0);
        border_win.refresh();
        let mut main_win = Window::new(1, 4, main_width - 2, height - 8);
        main_win.refresh();

        let mut output = Pad::new(100, 80);

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
            title: title_win,
            input: input_win,
        }
    }

    fn refresh(&self) {
        self.output.borrow_mut().render(&self.main);
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
}

impl Game {
    pub fn new() -> Game {
        Game {
            ui: UILayout::new(),
            world: World::new(),
            room: "NO_ROOM_SET".to_owned(),
        }
    }

    fn current_room(&self) -> &room::Room {
        self.world.get_room(&self.room).expect(&format!("Invalid room {}", self.room))
    }

    pub fn enter_room(&mut self, room: &str) {
        if !self.world.rooms.contains_key(room) {
            panic!("Entered room {} that doesn't exist", room);
        }
        self.room = room.into();
        self.ui.display(&self.current_room().description);
    }

    pub fn main(&mut self) {
        self.ui.refresh();
        loop {
            let input = self.ui.get_line(&self.ui.input).unwrap();

            match input.as_ref() {
                "exit" => break,
                "describe" => self.ui.display(&self.current_room().description),
                _ => {
                    self.ui.display(input.as_ref());
                },
            }
            self.ui.refresh();
        }
    }
}
