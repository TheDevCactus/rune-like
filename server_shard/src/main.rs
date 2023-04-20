use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};
use messaging::Message;
use shared::{area::Area, entity::Entity};

fn main() {
    let connections: std::sync::Arc<std::sync::Mutex<HashMap<SocketAddr, bool>>> =
        std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));
    let area = std::sync::Arc::new(std::sync::Mutex::new(Area::new()));

    let connections_clone = std::sync::Arc::clone(&connections);
    let _handle = std::thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind client socket");
        let mut buf = [0; 1];

        loop {
            let (_amt, src) = socket
                .recv_from(&mut buf)
                .expect("Could not read into buffer");

            match Message::from(buf[0]) {
                Message::Connect => {
                    println!("{} connected", src);
                    connections_clone.lock().unwrap().insert(src, true);
                    area.lock().unwrap().entities.push(Entity::new())
                }
                Message::Disconnect => {
                    println!("{} disconnected", src);
                    connections_clone.lock().unwrap().remove(&src);
                }
                Message::Ping => {
                    println!("{} pinged", src);
                },
                Message::Pong => {
                    println!("{} ponged", src);
                }
            }
        }
    });


    let socket = UdpSocket::bind("127.0.0.1:8081").expect("Could not bind client socket");
    let mut sleep_time = 0;
    let min_frame_time = 5000;
    loop {
        if sleep_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(sleep_time));
        }
        let tick_start = std::time::Instant::now();
        let message = [Message::Ping.into()]; 
        println!("Processing connections: {:?}", connections.lock().unwrap().len());
        // println!("World State: {:?}", world_state.positions);
        connections.lock().unwrap().iter().for_each(|(addr, _)| {
            match socket.send_to(&message, addr) {
                Ok(_) => {}
                Err(e) => println!("Error sending message: {}", e),
            }
        });
        sleep_time = min_frame_time - tick_start.elapsed().as_millis() as u64;
    }
}
