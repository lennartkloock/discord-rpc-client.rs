use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    io::Error as IoError,
    result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
};

use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    JsonError(JsonError),
    Timeout(ChannelTimeout),
    Conversion,
    SubscriptionFailed,
    ConnectionClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::JsonError(err)
    }
}

impl From<ChannelTimeout> for Error {
    fn from(err: ChannelTimeout) -> Self {
        Error::Timeout(err)
    }
}

pub type Result<T> = StdResult<T, Error>;
