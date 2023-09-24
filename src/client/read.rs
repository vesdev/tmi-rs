use super::{conn, Client};
use crate::irc::IrcMessage;
use futures_util::stream::Fuse;
use std::fmt::Display;
use tokio::io;
use tokio::io::{BufReader, ReadHalf};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;

pub type ReadStream = Fuse<LinesStream<BufReader<ReadHalf<conn::Stream>>>>;

impl Client {
  pub async fn message(&mut self) -> Result<IrcMessage, ReadError> {
    if let Some(message) = self.reader.next().await {
      let message = message?;
      Ok(IrcMessage::parse(&message).ok_or_else(|| ReadError::Parse(message))?)
    } else {
      Err(ReadError::StreamClosed)
    }
  }
}

#[derive(Debug)]
pub enum ReadError {
  Io(io::Error),
  Parse(String),
  StreamClosed,
}

impl ReadError {
  pub fn is_disconnect(&self) -> bool {
    match self {
      ReadError::StreamClosed => true,
      ReadError::Io(e)
        if matches!(
          e.kind(),
          io::ErrorKind::UnexpectedEof | io::ErrorKind::ConnectionAborted | io::ErrorKind::TimedOut
        ) =>
      {
        true
      }
      _ => false,
    }
  }
}

impl From<io::Error> for ReadError {
  fn from(value: io::Error) -> Self {
    Self::Io(value)
  }
}

impl Display for ReadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ReadError::Io(e) => write!(f, "failed to read message: {e}"),
      ReadError::Parse(s) => write!(f, "failed to read message: invalid message `{s}`"),
      ReadError::StreamClosed => write!(f, "failed to read message: stream closed"),
    }
  }
}

impl std::error::Error for ReadError {}