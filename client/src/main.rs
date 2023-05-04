extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use udp_lib::UdpClient;

pub fn main() {
    let (kill_signal_send, message_send_signal_send, message_receive_signal_recv, mut client) =
        UdpClient::spawn().unwrap();

    let networking_handle = std::thread::spawn(move || {
        let res = client.initialize("127.0.0.1:8080");
        match res {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to initialize client: {}", err);
            }
        }
    });

    loop {
        let res = message_send_signal_send.send((
            "This is a test message!!!!".to_string(),
            "127.0.0.1:8081".to_string(),
        ));
        match res {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to send message: {}", err);
            }
        }

        let res = message_receive_signal_recv.try_recv();
        match res {
            Ok(message) => {
                println!("Received message: {}", message);
            }
            Err(_) => {}
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // let render_handle: std::thread::JoinHandle<Result<(), ()>> = std::thread::spawn(move || {
    //     let sdl_context = sdl2::init().unwrap();
    //     let video_subsystem = sdl_context.video().unwrap();

    //     let window = video_subsystem
    //         .window("Runescape Clone", 800, 600)
    //         .resizable()
    //         .build()
    //         .unwrap();

    //     let mut canvas = window.into_canvas().build().unwrap();

    //     canvas.set_draw_color(Color::RGB(0, 255, 255));
    //     canvas.clear();
    //     canvas.present();
    //     let mut event_pump = sdl_context.event_pump().unwrap();
    //     let mut i = 0;
    //     'game_loop: loop {
    //         let x = message_receive_signal_recv.try_recv();
    //         match x {
    //             Ok(message) => {
    //                 println!("Received message: {}", message);
    //             }
    //             Err(_) => {}
    //         }

    //         i = (i + 1) % 255;
    //         canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //         canvas.clear();

    //         for event in event_pump.poll_iter() {
    //             match event {
    //                 Event::Quit { .. }
    //                 | Event::KeyDown {
    //                     keycode: Some(Keycode::Escape),
    //                     ..
    //                 } => break 'game_loop,
    //                 Event::KeyDown {
    //                     keycode: Some(Keycode::A),
    //                     ..
    //                 } => {
    //                     let res = message_send_signal_send.send((
    //                         "This is a test message!!!!".to_string(),
    //                         "127.0.0.1:8080".to_string(),
    //                     ));
    //                     if let Err(err) = res {
    //                         println!("Failed to send message: {}", err);
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }

    //         // The rest of the game loop goes here...

    //         canvas.present();

    //         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //     }

    //     Ok(())
    // });
    kill_signal_send.send(true).unwrap();
    let networking_results = networking_handle.join().unwrap();
    // let rendering_results = render_handle.join().unwrap();

    println!("Exiting game...");
}
