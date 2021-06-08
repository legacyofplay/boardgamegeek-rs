pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("received bad status code from BGG")]
  RequestFailed(u16),
  #[error("received bad response from BGG")]
  BadResponse,
  #[error("received invalid XML from BGG")]
  InvalidXML,
  #[error("could not connect to BGG")]
  ConnectionFailed,
  #[error("too many requests")]
  TooManyRequests,
}
