use crate::result::{Error, Result};
use backoff::{future, ExponentialBackoff};
use log::warn;
use reqwest::StatusCode;

const LOG_TARGET: &str = "boardgamegeek::Client";

// This "base" Client exists to handle the random idiosyncracies of BGG.
pub struct Client {
    http_client: reqwest::Client,
    no_redirect_client: reqwest::Client,
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
            http_client: reqwest::Client::builder().build().unwrap(),
            no_redirect_client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String> {
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

    pub async fn get_redirect_location(&self, url: &str) -> Result<Option<String>> {
        let result = self.no_redirect_client.get(url).send().await;

        if let Err(_err) = result {
            return Err(Error::ConnectionFailed);
        }

        let response = result.unwrap();
        match response.status() {
            StatusCode::FOUND => match response.headers().get("Location") {
                Some(v) => Ok(Some(v.to_str().unwrap().to_owned())),
                None => Ok(None),
            },
            code => Err(Error::RequestFailed(code.as_u16())),
        }
    }

    pub async fn get_with_202_check(&self, url: &str) -> Result<String> {
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
}
