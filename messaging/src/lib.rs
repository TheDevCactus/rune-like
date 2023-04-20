const PING: u8 = 0b00000000;
const CONNECT: u8 = 0b00000001;
const DISCONNECT: u8 = 0b00000010;
const PONG: u8 = 0b00000011;

pub enum Message {
    Ping,
    Pong,
    Connect,
    Disconnect,
}

impl Into<u8> for Message {
    fn into(self) -> u8 {
        match self {
            Message::Ping => PING,
            Message::Pong => PONG,
            Message::Connect => CONNECT,
            Message::Disconnect => DISCONNECT,
        }
    }
}

impl From<u8> for Message {
    fn from(byte: u8) -> Message {
        match byte {
            PING => Message::Ping,
            CONNECT => Message::Connect,
            DISCONNECT => Message::Disconnect,
            PONG => Message::Pong,
            _ => panic!("Invalid message type"),
        }
    }
}