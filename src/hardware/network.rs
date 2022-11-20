use std::path::{Path, PathBuf};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct NetworkInterface<'a> {
    defined_p: bool,
    present: bool,                                             // present
    device: PathBuf,                                           // device
    my_protocol: u8,                                           // my_protocol
    my_address: u32,                                           // my_address
    my_options: String,                                        // my_options
    other_address: [Option<&'a NetworkInterface<'a>>; 8], // Another addres
}

impl<'a> Default for NetworkInterface<'a> {
    fn default() -> Self {
        Self {
            defined_p: false,
            present: false,
            device: PathBuf::from(""),
            my_protocol: 0,
            my_address: 0xC0A8_0000, // 0xC0A8_0000 = 192.168.0.0
            my_options: String::from(""),
            other_address: [None, None, None, None, None, None, None, None],
        }
    }
}
