use std::{path::PathBuf, time};

use error::Result;
use named_pipe::PipeClient;

use super::base::Connection;

pub struct WindowsConnection {
    socket: PipeClient,
}

impl Connection for WindowsConnection {
    type Socket = PipeClient;

    fn connect() -> Result<Self> {
        let connection_name = Self::socket_path(0);
        let mut socket = PipeClient::connect(connection_name)?;
        socket.set_write_timeout(Some(time::Duration::from_secs(30)));
        socket.set_read_timeout(Some(time::Duration::from_secs(30)));
        Ok(Self { socket })
    }

    fn ipc_path() -> PathBuf {
        PathBuf::from(r"\\.\pipe\")
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }
}
