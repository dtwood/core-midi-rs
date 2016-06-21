use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_midi_sys::services::*;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std;
use time::Timespec;

#[derive(Debug)]
pub struct Client<'a>(MIDIClientRef, PhantomData<&'a ()>);
#[derive(Debug)]
pub struct Device<'a>(MIDIDeviceRef, PhantomData<&'a ()>);
#[derive(Debug)]
pub struct Endpoint<'a>(MIDIEndpointRef, bool, PhantomData<&'a ()>);
#[derive(Debug)]
pub struct Entity<'a>(MIDIEntityRef, PhantomData<&'a ()>);
#[derive(Debug)]
pub struct Port<'a>(MIDIPortRef, PhantomData<&'a ()>);

#[derive(Debug)]
pub struct Packet {
    pub time_stamp: Timespec,
    pub message: Message,
}

pub fn get_number_of_devices() -> u32 {
    unsafe { MIDIGetNumberOfDevices() }
}

pub fn get_device(index: u32) -> Device<'static> {
    assert!(index < get_number_of_devices());
    Device(unsafe { MIDIGetDevice(index) }, PhantomData)
}

pub fn get_devices() -> Vec<Device<'static>> {
    (0..get_number_of_devices()).map(|i| get_device(i)).collect()
}

impl<'a> Device<'a> {
    pub fn get_number_of_entities(&self) -> u32 {
        unsafe { MIDIDeviceGetNumberOfEntities(self.0) }
    }

    pub fn get_entity(&self, index: u32) -> Entity<'a> {
        assert!(index < self.get_number_of_entities());
        Entity(unsafe { MIDIDeviceGetEntity(self.0, index) }, PhantomData)
    }

    pub fn get_entities(&self) -> Vec<Entity<'a>> {
        (0..self.get_number_of_entities()).map(|i| self.get_entity(i)).collect()
    }
}

impl<'a> Entity<'a> {
    pub fn get_number_of_sources(&self) -> u32 {
        unsafe { MIDIEntityGetNumberOfSources(self.0) }
    }

    pub fn get_source(&self, index: u32) -> Endpoint<'a> {
        assert!(index < self.get_number_of_sources());
        Endpoint(unsafe { MIDIEntityGetSource(self.0, index) },
                 false,
                 PhantomData)
    }

    pub fn get_sources(&self) -> Vec<Endpoint<'a>> {
        (0..self.get_number_of_sources()).map(|i| self.get_source(i)).collect()
    }

    pub fn get_number_of_destinations(&self) -> u32 {
        unsafe { MIDIEntityGetNumberOfSources(self.0) }
    }

    pub fn get_destination(&self, index: u32) -> Endpoint<'a> {
        assert!(index < self.get_number_of_destinations());
        Endpoint(unsafe { MIDIEntityGetDestination(self.0, index) },
                 false,
                 PhantomData)
    }

    pub fn get_destinations(&self) -> Vec<Endpoint<'a>> {
        (0..self.get_number_of_destinations()).map(|i| self.get_destination(i)).collect()
    }

    pub fn get_device(&self) -> Device<'a> {
        let mut output = unsafe { mem::uninitialized() };

        let result = unsafe { MIDIEntityGetDevice(self.0, &mut output) };
        assert_eq!(result, 0);

        Device(output, PhantomData)
    }
}

pub struct Note(i8);

impl std::fmt::Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 % 12 {
            0 => try!(f.write_str("C")),
            1 => try!(f.write_str("C♯")),
            2 => try!(f.write_str("D")),
            3 => try!(f.write_str("E♭")),
            4 => try!(f.write_str("E")),
            5 => try!(f.write_str("F")),
            6 => try!(f.write_str("F♯")),
            7 => try!(f.write_str("G")),
            8 => try!(f.write_str("G♯")),
            9 => try!(f.write_str("A")),
            10 => try!(f.write_str("B♭")),
            11 => try!(f.write_str("B")),
            _ => unreachable!(),
        };

        try!(f.write_fmt(format_args!("{}", self.0 / 12 - 1)));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Message {
    ActiveSense,
    BeatClock,
    NoteDown(Note, u8),
    ContinuousController(u8, i8),
    Invalid(Vec<u8>),
    Other(Vec<u8>),
}

impl Message {
    pub fn vec_from_slice(data: &[u8]) -> Vec<Message> {
        let mut output = Vec::new();

        let mut data = data.iter();

        loop {
            let next = match data.next() {
                Some(&0xB0) => {
                    match (data.next(), data.next()) {
                        (Some(&index), Some(&value)) => {
                            Message::ContinuousController(index, value as i8)
                        }
                        (Some(&v1), None) => Message::Invalid(vec![0xB0, v1]),
                        (None, None) => Message::Invalid(vec![0xB0]),
                        (None, Some(_)) => unreachable!(),
                    }
                }
                Some(&0x90) => {
                    match (data.next(), data.next()) {
                        (Some(&note), Some(&velocity)) => {
                            Message::NoteDown(Note(note as i8), velocity)
                        }
                        (Some(&v1), None) => Message::Invalid(vec![0x90, v1]),
                        (None, None) => Message::Invalid(vec![0x90]),
                        (None, Some(_)) => unreachable!(),
                    }
                }
                Some(&0xF8) => Message::BeatClock,
                Some(&0xFE) => Message::ActiveSense,
                Some(&x) => {
                    let mut message = Vec::new();
                    message.push(x);
                    message.extend(data);
                    output.push(Message::Other(message));
                    break;
                }
                None => break,
            };

            output.push(next);
        }

        output
    }
}

impl Packet {
    fn vec_from_list(packet_list: &MIDIPacketList) -> Vec<Packet> {
        let packet_count: usize = packet_list.numPackets as usize;
        let body: *const MIDIPacket = &packet_list.packet as *const c_void as *const MIDIPacket;

        let mut output = Vec::new();
        let mut offset = 0;

        for _ in 0..packet_count {
            let packet_ptr = (body as usize + offset) as *const MIDIPacket;
            let packet: &MIDIPacket = unsafe { &*packet_ptr };
            let header_size = (&packet.data as *const _ as usize) -
                              (&packet.timeStamp as *const _ as usize);
            let data = unsafe {
                ::std::slice::from_raw_parts(&packet.data as *const _ as *const u8,
                                             packet.length as usize)
            };

            for message in Message::vec_from_slice(data) {
                output.push(Packet {
                    time_stamp: Timespec::new(0, 0),
                    message: message,
                });
            }


            offset += header_size + packet.length as usize;
        }

        output
    }
}

impl<'a> Client<'a> {
    pub fn new<'n, F: FnMut(MIDINotification)>(name: &'n CFString,
                                               callback: &'n mut F)
                                               -> Client<'n> {
        unsafe extern "C" fn callback_wrapper_func<F: FnMut(MIDINotification)>(
                notification: *const MIDINotification,
                callback: *mut c_void) {
            let _ = std::panic::catch_unwind(|| {
                debug_assert!(notification != ptr::null());
                debug_assert!(callback != ptr::null_mut());

                let callback = &mut *(callback as *mut F);

                callback(*notification);
            });
        }

        let callback_wrapper = Some(callback_wrapper_func::<F> as unsafe extern "C" fn(_, _));
        let callback_pointer = callback as *mut F as *mut c_void;

        let mut output = unsafe { mem::uninitialized() };
        let result = unsafe {
            MIDIClientCreate(name.as_concrete_TypeRef(),
                             callback_wrapper,
                             callback_pointer,
                             &mut output)
        };
        assert_eq!(result, 0);

        Client(output, PhantomData)
    }

    pub fn create_source<'n>(&mut self, name: &'n CFString) -> Endpoint<'n> {
        let mut output = unsafe { mem::uninitialized() };
        let result = unsafe { MIDISourceCreate(self.0, name.as_concrete_TypeRef(), &mut output) };
        assert_eq!(result, 0);

        Endpoint(output, true, PhantomData)
    }

    pub fn create_input_port<'n, F: FnMut(Vec<Packet>)>(&mut self,
                                                        name: &'n CFString,
                                                        callback: &'n mut F)
                                                        -> Port<'n> {
        unsafe extern "C" fn callback_wrapper_func<F: FnMut(Vec<Packet>)>(
                packet_list: *const MIDIPacketList,
                callback: *mut c_void,
                source_data: *mut c_void) {
            let _ = std::panic::catch_unwind(|| {
                debug_assert!(packet_list != ptr::null());
                debug_assert!(callback != ptr::null_mut());
                debug_assert!(source_data == ptr::null_mut());

                let callback = &mut *(callback as *mut F);
                let packet_list = Packet::vec_from_list(&*packet_list);

                callback(packet_list);
            });
        }

        let callback_wrapper = Some(callback_wrapper_func::<F> as unsafe extern "C" fn(_, _, _));
        let callback_pointer = callback as *mut F as *mut c_void;

        let mut output = unsafe { mem::uninitialized() };
        let result = unsafe {
            MIDIInputPortCreate(self.0,
                                name.as_concrete_TypeRef(),
                                callback_wrapper,
                                callback_pointer,
                                &mut output)
        };
        assert_eq!(result, 0);

        Port(output, PhantomData)
    }

    pub fn create_output_port<'n>(&mut self, name: &'n CFString) -> Port<'n> {
        let mut output = unsafe { mem::uninitialized() };
        let result =
            unsafe { MIDIOutputPortCreate(self.0, name.as_concrete_TypeRef(), &mut output) };
        assert_eq!(result, 0);

        Port(output, PhantomData)
    }
}

impl<'a> Drop for Client<'a> {
    fn drop(&mut self) {
        unsafe {
            MIDIClientDispose(self.0);
        }
    }
}


impl<'a> Drop for Endpoint<'a> {
    fn drop(&mut self) {
        if self.1 {
            unsafe {
                MIDIEndpointDispose(self.0);
            }
        }
    }
}

impl<'a> Port<'a> {
    pub fn connect_source(&mut self, source: &Endpoint) {
        let result = unsafe { MIDIPortConnectSource(self.0, source.0, ptr::null_mut()) };
        assert_eq!(result, 0);
    }

    pub fn disconnect_source(&mut self, source: &Endpoint) {
        let result = unsafe { MIDIPortDisconnectSource(self.0, source.0) };
        assert_eq!(result, 0);
    }
}

impl<'a> Drop for Port<'a> {
    fn drop(&mut self) {
        unsafe {
            MIDIPortDispose(self.0);
        }
    }
}
