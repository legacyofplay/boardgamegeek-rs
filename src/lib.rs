mod client;
mod collection;
mod parsers {
    pub mod boardgame;
}
mod protocol;
mod result;
mod scraping;
mod thing;
mod xml;

pub use {
    client::Client,
    collection::{Collection, CollectionType},
    result::Result,
    thing::{Link, Thing},
};
