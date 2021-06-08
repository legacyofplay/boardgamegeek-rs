use crate::collection::{Collection, CollectionParser, CollectionType};
use crate::result::{Error, Result};
use crate::thing::{Thing, ThingParser};
use backoff::{future, ExponentialBackoff};
use log::warn;
use reqwest::StatusCode;

const LOG_TARGET: &str = "boardgamegeek::Client";

pub struct Client {
  http_client: reqwest::Client,
}

async fn collect(response: reqwest::Response) -> Result<String> {
  match response.text().await {
    Ok(text) => Ok(text),
    Err(_err) => Err(Error::BadResponse),
  }
}

impl Client {
  pub fn new() -> Self {
    Self {
      http_client: reqwest::Client::new(),
    }
  }

  async fn get(&self, url: &str) -> Result<String> {
    future::retry(ExponentialBackoff::default(), || async {
      let result = self.http_client.get(url).send().await;

      if let Err(_err) = result {
        return Err(backoff::Error::Permanent(Error::ConnectionFailed));
      }

      let response = result.unwrap();
      match response.status() {
        StatusCode::TOO_MANY_REQUESTS => {
          warn!(target: LOG_TARGET, "Received 429: {:?}", response);
          Err(backoff::Error::Transient(Error::TooManyRequests))
        }
        StatusCode::OK => Ok(collect(response).await?),
        code => Err(backoff::Error::Permanent(Error::RequestFailed(
          code.as_u16(),
        ))),
      }
    })
    .await
  }

  async fn get_with_202_check(&self, url: &str) -> Result<String> {
    future::retry(backoff::ExponentialBackoff::default(), || async {
      match self.get(url).await {
        Err(Error::RequestFailed(202)) => {
          warn!(target: LOG_TARGET, "Received 202. Retrying...");
          Err(backoff::Error::Transient(Error::RequestFailed(202)))
        }

        Err(e) => Err(backoff::Error::Permanent(e)),
        Ok(r) => Ok(r),
      }
    })
    .await
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

    let result = self.get_with_202_check(url.as_str()).await?;

    match CollectionParser::new().parse(result.as_bytes()) {
      Ok(collection) => Ok(collection),
      Err(_err) => Err(Error::InvalidXML),
    }
  }

  pub async fn get_thing(&self, id: &str) -> Result<Thing> {
    let result = self
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
