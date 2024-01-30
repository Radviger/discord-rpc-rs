use std::{
    thread,
    sync::{
        Arc,
    },
    time,
    io::ErrorKind
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;

use super::{
    Connection,
    SocketConnection,
};
use models::{Message, Handshake};
use error::{Result, Error};


type Tx = Sender<Message>;
type Rx = Receiver<Message>;

#[derive(Clone)]
pub struct Manager {
    connection: Arc<Option<Mutex<SocketConnection>>>,
    client_id: u64,
    outbound: (Rx, Tx),
    inbound: (Rx, Tx),
    handshake_completed: bool
}

impl Manager {
    pub fn new(client_id: u64) -> Self {
        let connection = Arc::new(None);
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

    pub fn start_loop<C, S, H>(&mut self, connection_error: C, send_error: S, handshake_fn: H) where C: Fn(Error), S: Fn(Error), H: Fn(Handshake) {
        let mut inbound = self.inbound.1.clone();
        let outbound = self.outbound.0.clone();

        loop {
            let connection = self.connection.clone();

            match *connection {
                Some(ref conn) => {
                    let mut connection = conn.lock();
                    match send_and_receive(&mut *connection, &mut inbound, &outbound) {
                        Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(Error::IoError(_)) | Err(Error::ConnectionClosed) => self.disconnect(),
                        Err(why) => send_error(why),
                        _ => (),
                    }

                    thread::sleep(time::Duration::from_millis(500));
                },
                None => {
                    match self.connect() {
                        Err(err) => {
                            match err {
                                Error::IoError(ref err) if err.kind() == ErrorKind::ConnectionRefused => (),
                                why => connection_error(why),
                            }
                            thread::sleep(time::Duration::from_secs(10));
                        },
                        Ok(handshake) => {
                            handshake_fn(handshake);
                            self.handshake_completed = true;
                        },
                    }
                }
            }
        }
    }

    pub fn send(&self, message: Message) -> Result<()> {
        self.outbound.1.send(message).unwrap();
        Ok(())
    }

    pub fn recv(&self) -> Result<Message> {
        let message = self.inbound.0.recv().unwrap();
        Ok(message)
    }

    fn connect(&mut self) -> Result<Handshake> {
        debug!("Connecting");

        let mut new_connection = SocketConnection::connect()?;

        debug!("Performing handshake");
        let handshake = new_connection.handshake(self.client_id)?;
        debug!("Handshake completed");

        self.connection = Arc::new(Some(Mutex::new(new_connection)));

        debug!("Connected");

        Ok(handshake)
    }

    fn disconnect(&mut self) {
        self.handshake_completed = false;
        self.connection = Arc::new(None);
    }

}

fn send_and_receive(connection: &mut SocketConnection, inbound: &mut Tx, outbound: &Rx) -> Result<()> {
    while let Ok(msg) = outbound.try_recv() {
        connection.send(msg)?;
    }

    let msg = connection.recv()?;
    inbound.send(msg).expect("Failed to send received data");

    Ok(())
}
