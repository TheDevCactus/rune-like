use std::sync::mpsc::{Receiver, Sender};

pub const BUFFER_SIZE: usize = 1024;

pub struct UdpClient {
    initialized: bool,
    socket: Option<std::net::UdpSocket>,
    connections: std::collections::HashMap<std::net::SocketAddr, bool>,
    kill_signal: std::sync::mpsc::Receiver<bool>,
    message_send_signal: std::sync::mpsc::Receiver<(String, String)>,
    message_receive_signal: std::sync::mpsc::Sender<String>,
}

//
// UDP Client
// The following client can be spun up in a thread and will handle all UDP operations
// such as connecting, sending, and receiving messages.
//
// The client will also handle all connections and will store them in a HashMap
//
// We utilize MPSC primitives to communicate with the main thread

impl UdpClient {
    pub fn spawn() -> Result<
        (
            Sender<bool>,
            Sender<(String, String)>,
            Receiver<String>,
            UdpClient,
        ),
        (),
    > {
        let (kill_signal_send, kill_signal_recv) = std::sync::mpsc::channel();
        let (message_send_signal_send, message_send_signal_recv) = std::sync::mpsc::channel();
        let (message_receive_signal_send, message_receive_signal_recv) = std::sync::mpsc::channel();

        let client = UdpClient {
            initialized: false,
            socket: None,
            connections: std::collections::HashMap::new(),
            kill_signal: kill_signal_recv,
            message_send_signal: message_send_signal_recv,
            message_receive_signal: message_receive_signal_send,
        };

        return Ok((
            kill_signal_send,
            message_send_signal_send,
            message_receive_signal_recv,
            client,
        ));
    }
    pub fn initialize(&mut self, addr: &str) -> Result<(), String> {
        let socket = std::net::UdpSocket::bind(addr).expect("Failed to bind socket");
        self.socket = Some(socket);
        self.initialized = true;
        return Ok(());
    }

    pub fn send_message(&self, message: &str, addr: &str) -> Result<(), String> {
        if !self.initialized {
            return Err("Client not initialized".to_string());
        }

        return match self
            .socket
            .as_ref()
            .unwrap()
            .send_to(message.as_bytes(), addr)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to send message: {}", err)),
        };
    }

    pub fn broadcast_message(&self, message: &str) -> Result<(), String> {
        if !self.initialized {
            return Err("Client not initialized".to_string());
        }

        let results = self
            .connections
            .keys()
            .map(|addr| {
                self.socket
                    .as_ref()
                    .unwrap()
                    .send_to(message.as_bytes(), addr)
            })
            .filter(|x| x.is_err())
            .collect::<Vec<_>>();

        return match results.len() != 0 {
            true => Err("Failed to send message to all clients".to_string()),
            false => Ok(()),
        };
    }

    pub fn start_processing(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Err("Client not initialized".to_string());
        }

        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        'networking_loop: loop {
            if let Ok((msg, addr)) = self.message_send_signal.try_recv() {
                let res = self.send_message(&msg, &addr);
                if let Err(err) = res {
                    return Err(err);
                }
            }

            if let Ok(true) = self.kill_signal.try_recv() {
                break 'networking_loop;
            }

            let (amt, src) = self
                .socket
                .as_ref()
                .unwrap()
                .recv_from(&mut buf)
                .expect("Failed to receive message");

            self.connections.entry(src).or_insert(true);

            let msg = std::str::from_utf8(&buf[..amt]).expect("Failed to parse message");
            let res = self.message_receive_signal.send(msg.to_string());

            if let Err(_) = res {
                return Err("Failed to send message to main thread".to_string());
            }
        }

        return Ok(());
    }
}
