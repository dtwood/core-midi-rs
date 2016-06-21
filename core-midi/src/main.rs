#![feature(const_fn)]

extern crate ctrlc;
extern crate core_midi;
extern crate core_foundation;

use core_midi::*;
use core_foundation::string::CFString;

use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::Relaxed);
    });

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

    while RUNNING.load(Ordering::Relaxed) {
        std::thread::yield_now();
    }
}
