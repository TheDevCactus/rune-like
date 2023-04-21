// All messages are sent within a single u64, so we can use bit packing to encode and decode them.

pub enum Message {
    Connect,
    Disconnect,
    Ping,
    Pong,
    MoveTo { x: u8, y: u8 },
    Interact { id: u8 },
}
