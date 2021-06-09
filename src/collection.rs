use crate::xml::*;
use std::collections::HashMap;
use xml::reader::{EventReader, Result, XmlEvent};

pub enum CollectionType {
    BoardGames,
    BoardGameExpansions,
}

#[derive(Debug)]
pub struct CollectionItem {
    pub id: i64,
    pub statuses: HashMap<String, i64>,
}

impl CollectionItem {
    fn new(id: i64) -> Self {
        Self {
            id: id,
            statuses: HashMap::new(),
        }
    }

    pub fn is_owned(&self) -> bool {
        let own = self.statuses.get("own");

        if own == None {
            return false;
        }

        return *own.unwrap() > 0;
    }
}

#[derive(Debug)]
pub struct Collection {
    pub items: Vec<CollectionItem>,
}

impl Collection {
    fn new() -> Collection {
        Collection { items: Vec::new() }
    }

    pub fn parse<T: std::io::Read>(bytes: T) -> Result<Collection> {
        CollectionParser::new().parse(bytes)
    }
}

pub struct CollectionParser {
    collection: Collection,
    stack: Vec<String>,
}

impl CollectionParser {
    pub fn new() -> CollectionParser {
        CollectionParser {
            collection: Collection::new(),
            stack: Vec::new(),
        }
    }

    pub fn parse<T: std::io::Read>(mut self, bytes: T) -> xml::reader::Result<Collection> {
        let parser = EventReader::new(bytes);

        for event in parser {
            self.on_event(event?);
        }

        Ok(self.collection)
    }

    fn on_event(&mut self, event: xml::reader::XmlEvent) {
        match event {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                self.stack.push(name.local_name.clone());
                self.on_start_element(name, attributes);
            }
            XmlEvent::EndElement { .. } => {
                self.stack.pop();
            }
            _ => {}
        }
    }

    fn on_start_element(
        &mut self,
        name: xml::name::OwnedName,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        match name.local_name.as_str() {
            "item" => {
                if let Some(id) = num_for_attr(&attributes, "objectid") {
                    self.collection.items.push(CollectionItem::new(id));
                }
            }
            "status" => {
                if let Some(ref mut last) = self.collection.items.last_mut() {
                    // own="1" prevowned="0" fortrade="0" want="0" wanttoplay="0" wanttobuy="0" wishlist="0"  preordered="0" lastmodified="2020-07-07 23:16:17" />
                    if let Some(own) = num_for_attr(&attributes, "own") {
                        last.statuses.insert(String::from("own"), own);
                    }
                }
            }
            _ => {}
        }
    }
}
