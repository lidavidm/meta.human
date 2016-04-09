use std::collections::HashMap;

use yaml_rust;

use assets;

pub struct Room {
    pub name: String,
    pub description: String,
    // doors: ,
    // characters: ,
    // contents: ,
}

impl assets::Decodable for (String, Room) {
    fn decode(doc: &yaml_rust::Yaml) -> assets::Result<(String, Room)> {
        let id = get_field!(doc, "id", as_str);
        let name = get_field!(doc, "name", as_str);
        let desc = get_field!(doc, "description", as_str);

        Ok((id.to_owned(), Room {
            name: name.to_owned(),
            description: desc.to_owned(),
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
