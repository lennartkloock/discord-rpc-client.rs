extern crate byteorder;
extern crate bytes;
extern crate crossbeam_channel;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;
#[cfg(windows)]
extern crate named_pipe;
extern crate parking_lot;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate uuid;

pub use client::Client;
pub use connection::{Connection, SocketConnection};

#[macro_use]
mod macros;
pub mod client;
mod connection;
mod error;
pub mod models;
mod utils;
