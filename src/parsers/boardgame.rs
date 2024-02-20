use crate::xml::*;
use xml::reader::{EventReader, Result, XmlEvent};

#[derive(Debug)]
pub struct Link {
    pub id: i64,
    pub link_type: String,
    pub value: String,
}

#[derive(Debug)]
pub struct BoardGame {
    pub id: i64,
    pub primary_name: String,
    pub description: String,
    pub year_published: i32,
    pub thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub minage: i32,
    pub owners: i32,
    pub players: std::ops::Range<i32>,
    pub playtime: std::ops::Range<i32>,
    pub weight: f32,
    pub rating: f32,
    pub usersrated: i32,
    pub trading: i32,
    pub wanting: i32,
    pub wishing: i32,
    pub numcomments: i32,
    pub numweights: i32,
    pub links: Vec<Link>,
}

impl BoardGame {
    fn new() -> BoardGame {
        BoardGame {
            id: 0,
            primary_name: String::new(),
            description: String::new(),
            year_published: 0,
            thumbnail_url: None,
            image_url: None,
            minage: 0,
            owners: 0,
            players: 0..0,
            playtime: 0..0,
            weight: 0.0,
            rating: 0.0,
            usersrated: 0,
            trading: 0,
            wanting: 0,
            wishing: 0,
            numcomments: 0,
            numweights: 0,
            links: Vec::new(),
        }
    }
}

pub struct BoardGameParser {
    stack: Vec<String>,
    thing: BoardGame,
}

impl BoardGameParser {
    pub fn new() -> BoardGameParser {
        BoardGameParser {
            stack: Vec::new(),
            thing: BoardGame::new(),
        }
    }

    pub fn parse<T: std::io::Read>(mut self, bytes: T) -> Result<BoardGame> {
        let parser = EventReader::new(bytes);

        for event in parser {
            self.on_event(event?);
        }

        self.scrub();

        Ok(self.thing)
    }

    fn scrub(&mut self) {
        if self.thing.players.end < self.thing.players.start {
            self.thing.players.end = self.thing.players.start;
        }
        if self.thing.playtime.end < self.thing.playtime.start {
            self.thing.playtime.end = self.thing.playtime.start;
        }
    }

    fn on_event(&mut self, event: xml::reader::XmlEvent) {
        match event {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                self.stack.push(name.local_name.clone());
                self.on_start_element(name, attributes)
            }
            XmlEvent::EndElement { .. } => {
                self.stack.pop();
            }
            XmlEvent::Characters(chars) => self.on_characters(chars),
            _ => {}
        }
    }

    fn on_start_element(
        &mut self,
        name: xml::name::OwnedName,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        println!("OSE: {:?} => {:?}", name, attributes);

        match name.local_name.as_str() {
            "boardgame" => {
                if let Some(id) = num_for_attr(&attributes, "objectid") {
                    self.thing.id = id;
                }
            }
            "name" => {
                if string_for_attr(&attributes, "type") == Some("primary".into()) {
                    if let Some(value) = string_for_attr(&attributes, "value") {
                        self.thing.primary_name = value;
                    }
                }

                // HACK(schoon): Deal with the mix of attrs and chars by
                // swapping the name on the top of the stack.
                if string_for_attr(&attributes, "primary") == Some("true".into()) {
                    self.stack.pop();
                    self.stack.push(String::from("primary_name"));
                }
            }
            "yearpublished" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.year_published = value;
                }
            }
            "minplayers" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.players.start = value;
                }
            }
            "maxplayers" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.players.end = value;
                }
            }
            "minplaytime" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.playtime.start = value;
                }
            }
            "maxplaytime" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.playtime.end = value;
                }
            }
            "minage" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.minage = value;
                }
            }
            "link" => {
                let mut link = Link {
                    id: 0,
                    link_type: String::new(),
                    value: String::new(),
                };

                if let Some(id) = num_for_attr(&attributes, "id") {
                    link.id = id;
                }
                if let Some(link_type) = string_for_attr(&attributes, "type") {
                    link.link_type = link_type;
                }
                if let Some(value) = string_for_attr(&attributes, "value") {
                    link.value = value;
                }

                self.thing.links.push(link);
            }
            "usersrated" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.usersrated = value;
                }
            }
            "average" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = f32_for_attr(&attributes, "value") {
                    self.thing.rating = value;
                }
            }
            "averageweight" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = f32_for_attr(&attributes, "value") {
                    self.thing.weight = value;
                }
            }
            "owned" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.owners = value;
                }
            }
            "trading" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.trading = value;
                }
            }
            "wanting" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.wanting = value;
                }
            }
            "wishing" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.wishing = value;
                }
            }
            "numcomments" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.numcomments = value;
                }
            }
            "numweights" if self.stack[self.stack.len() - 2] == "ratings" => {
                if let Some(value) = num_for_attr(&attributes, "value") {
                    self.thing.numweights = value;
                }
            }
            _ => {}
        }
    }

    fn on_characters(&mut self, chars: String) {
        match self.stack[self.stack.len() - 1].as_str() {
            "description" => self.thing.description = chars,
            "image" => self.thing.image_url = Some(chars),
            "thumbnail" => self.thing.thumbnail_url = Some(chars),
            "yearpublished" => self.thing.year_published = num_for_chars(chars).unwrap_or(0),
            "minplayers" => self.thing.players.start = num_for_chars(chars).unwrap_or(0),
            "maxplayers" => self.thing.players.end = num_for_chars(chars).unwrap_or(0),
            "minplaytime" => self.thing.playtime.start = num_for_chars(chars).unwrap_or(0),
            "maxplaytime" => self.thing.playtime.end = num_for_chars(chars).unwrap_or(0),
            "age" => self.thing.minage = num_for_chars(chars).unwrap_or(0),
            // TODO(schoon): Parse links.
            // This is a fake tag name backed by `name primary="true"`.
            "primary_name" => self.thing.primary_name = chars,
            _ => {}
        }
    }
}
