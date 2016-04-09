use std::collections::HashMap;

use yaml_rust;

use assets;

pub struct Room {
    pub name: String,
    pub description: String,
    // phrase : room name
    pub doors: HashMap<String, String>,
    // characters: ,
    // contents: ,
}

impl Room {
    pub fn find_door(&self, name: &str) -> Option<&String> {
        self.doors.get(name)
    }

    pub fn annotated_description(&self) -> String {
        let mut description = self.description.clone();
        for door_name in self.doors.keys() {
            description = description.replace(door_name, &format!("[{}]", door_name));
        }

        description
    }
}

impl assets::Decodable for (String, Room) {
    fn decode(doc: &yaml_rust::Yaml) -> assets::Result<(String, Room)> {
        let id = get_field!(doc, "id", as_str);
        let name = get_field!(doc, "name", as_str);
        let desc = get_field!(doc, "description", as_str);
        let exits = get_field!(doc, "exits", as_hash);

        let mut doors = HashMap::new();

        for (room_name, exit_names) in exits.iter() {
            let room_name = as_value!(room_name, as_str).to_owned();
            for exit_name in as_value!(exit_names, as_vec).iter() {
                doors.insert(as_value!(exit_name, as_str).to_owned(), room_name.clone());
            }
        }

        Ok((id.to_owned(), Room {
            name: name.to_owned(),
            description: desc.to_owned(),
            doors: doors,
        }))
    }
}

pub fn load_rooms(docs: Vec<yaml_rust::Yaml>) -> assets::Result<HashMap<String, Room>> {
    let mut result = HashMap::with_capacity(docs.len());

    for room in docs.iter().map(assets::Decodable::decode) {
        let (key, room) = try!(room);
        result.insert(key, room);
    }

    Ok(result)
}
