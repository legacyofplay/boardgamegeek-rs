use crate::collection::{Collection, CollectionParser, CollectionType};
use crate::protocol;
use crate::result::{Error, Result};
use crate::scraping;
use crate::thing::{Thing, ThingParser};

pub struct Client {
  base_client: protocol::Client,
}

impl Client {
  pub fn new() -> Self {
    Self {
      base_client: protocol::Client::new(),
    }
  }

  pub async fn get_collection(
    &self,
    username: &str,
    subtype: CollectionType,
  ) -> Result<Collection> {
    let url = format!(
      "https://www.boardgamegeek.com/xmlapi2/collection?username={}&{}",
      username,
      match subtype {
        CollectionType::BoardGames => "excludesubtype=boardgameexpansion",
        CollectionType::BoardGameExpansions => "subtype=boardgameexpansion",
      }
    );

    let result = self.base_client.get_with_202_check(url.as_str()).await?;

    match CollectionParser::new().parse(result.as_bytes()) {
      Ok(collection) => Ok(collection),
      Err(_err) => Err(Error::InvalidXML),
    }
  }

  pub async fn get_random_boardgame_id(&self) -> Result<String> {
    let result = self
      .base_client
      .get_redirect_location("https://www.boardgamegeek.com/boardgame/random")
      .await?;

    match result {
      Some(location) => Ok(scraping::id_from_url(&location)),
      None => Err(Error::BadResponse),
    }
  }

  pub async fn get_thing(&self, id: &str) -> Result<Thing> {
    let result = self
      .base_client
      .get(
        format!(
          "https://www.boardgamegeek.com/xmlapi2/thing?id={}&stats=true",
          id
        )
        .as_str(),
      )
      .await?;

    match ThingParser::new().parse(result.as_bytes()) {
      Ok(thing) => Ok(thing),
      Err(_err) => Err(Error::InvalidXML),
    }
  }
}
