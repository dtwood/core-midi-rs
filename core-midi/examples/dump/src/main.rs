extern crate core_foundation;
extern crate core_midi;
extern crate ctrlc;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use core_foundation::string::CFString;
use core_midi::*;

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    {
        let running = running.clone();

        ctrlc::set_handler(move || {
            running.store(false, Ordering::Relaxed);
        });
    }

    let running = running.clone();

    let name = CFString::new("rust");

    let panic = |_| panic!();
    let mut client = Client::new(&name, &panic);

    let cb = |packets: Vec<Packet>| {
        for packet in packets {
            match packet.message {
                Message::ActiveSense => continue,
                Message::BeatClock => continue,
                _ => {}
            };
            println!("{:?}", packet);
        }
    };
    let mut port = client.create_input_port(&name, &cb);

    for device in get_devices() {
        for entity in device.get_entities() {
            for source in entity.get_sources() {
                port.connect_source(&source);
            }
        }
    }

    while running.load(Ordering::Relaxed) {
        std::thread::yield_now();
    }
}
