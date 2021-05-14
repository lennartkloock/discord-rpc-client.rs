use std::{io::ErrorKind, sync::Arc, thread, time};

use crossbeam_channel::{Receiver, Sender, unbounded};
use parking_lot::Mutex;

use error::{Error, Result};
use models::Message;

use super::{Connection, SocketConnection};

type Tx = Sender<Message>;
type Rx = Receiver<Message>;

#[derive(Clone)]
pub struct Manager {
    connection: Arc<Mutex<Option<SocketConnection>>>,
    client_id: u64,
    outbound: (Rx, Tx),
    inbound: (Rx, Tx),
    handshake_completed: bool,
}

impl Manager {
    pub fn new(client_id: u64) -> Self {
        let connection = Arc::new(Mutex::new(None));
        let (sender_o, receiver_o) = unbounded();
        let (sender_i, receiver_i) = unbounded();

        Self {
            connection,
            client_id,
            handshake_completed: false,
            inbound: (receiver_i, sender_i),
            outbound: (receiver_o, sender_o),
        }
    }

    pub fn start(&mut self, retries: u32) {
        let manager_inner = self.clone();
        thread::spawn(move || {
            send_and_receive_loop(manager_inner, retries);
        });
    }

    pub fn send(&self, message: Message) -> Result<()> {
        self.outbound
            .1
            .send(message)
            .map_err(|err| Error::SendError(err))?;
        Ok(())
    }

    pub fn recv(&self) -> Result<Message> {
        let message = self.inbound.0.recv().map_err(|err| Error::RecvError(err))?;
        Ok(message)
    }

    fn connect(&mut self) -> Result<()> {
        if self.is_connected() {
            return Ok(());
        }

        debug!("Connecting");

        let mut new_connection = SocketConnection::connect()?;

        debug!("Performing handshake");
        new_connection.handshake(self.client_id)?;
        debug!("Handshake completed");

        *self.connection.lock() = Some(new_connection);

        debug!("Connected: {:?}", self.is_connected());

        Ok(())
    }

    fn disconnect(&mut self) {
        debug!("Disconnected");
        self.handshake_completed = false;
        *self.connection.lock() = None;
    }

    pub fn is_connected(&self) -> bool {
        self.connection.lock().is_some()
    }
}

fn send_and_receive_loop(mut manager: Manager, retries: u32) {
    debug!("Starting sender loop");

    let mut inbound = manager.inbound.1.clone();
    let outbound = manager.outbound.0.clone();

    let mut err_counter = 0;
    while err_counter < retries {
        let connection = Arc::clone(&manager.connection);

        let mut lock = connection.lock();
        match *lock {
            Some(ref mut conn) => {
                match send_and_receive(conn, &mut inbound, &outbound) {
                    Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(Error::IoError(_)) | Err(Error::ConnectionClosed) => manager.disconnect(),
                    Err(why) => error!("error: {}", why),
                    _ => (),
                }

                drop(lock);
                thread::sleep(time::Duration::from_millis(500));
            }
            None => {
                drop(lock);
                match manager.connect() {
                    Err(err) => {
                        err_counter += 1;
                        match err {
                            Error::IoError(ref err)
                            if err.kind() == ErrorKind::ConnectionRefused =>
                                {
                                    warn!(
                                        "(Try {}/{}) Failed to connect: connection refused",
                                        err_counter, retries
                                    );
                                }
                            why => error!(
                                "(Try {}/{}) Failed to connect: {:?}",
                                err_counter, retries, why
                            ),
                        }
                        thread::sleep(time::Duration::from_secs(5));
                    }
                    _ => manager.handshake_completed = true,
                }
            }
        };
    }

    debug!("Ending sender loop");
}

fn send_and_receive(
    connection: &mut SocketConnection,
    inbound: &mut Tx,
    outbound: &Rx,
) -> Result<()> {
    while let Ok(msg) = outbound.try_recv() {
        connection.send(msg).expect("Failed to send outgoing data");
    }

    let msg = connection.recv()?;
    inbound.send(msg).expect("Failed to send received data");

    Ok(())
}
