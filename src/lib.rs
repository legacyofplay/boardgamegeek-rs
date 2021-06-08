mod client;
mod collection;
mod result;
mod thing;
mod xml;

pub use {
  client::Client,
  collection::{Collection, CollectionType},
  result::Result,
  thing::Thing,
};
