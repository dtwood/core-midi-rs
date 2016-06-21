extern crate core_midi;
extern crate core_foundation;

use core_midi::*;
use core_foundation::string::CFString;

#[test]
fn count_devices() {
    println!("{}", get_number_of_devices());
}

#[test]
fn create() {
    let name = CFString::new("rust");

    let mut a = 0;
    let mut nop = |_| a += 1;
    let mut client = Client::new_mut(&name, &mut nop);
    let endpoint = client.create_source(&name);

    let mut cb = |packet_list| println!("{:?}", packet_list);
    let mut port = client.create_input_port(&name, &mut cb);

    port.connect_source(&endpoint);
    port.disconnect_source(&endpoint);
}
