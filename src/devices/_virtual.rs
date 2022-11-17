use std::io;

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, EventType, InputEvent, Key,
};

pub fn new() -> Result<VirtualDevice, io::Error> {
    let keys: AttributeSet<Key> = (1..248).map(Key::new).collect();

    VirtualDeviceBuilder::new()?
        .name("Virtual Keyboard")
        .with_keys(&keys)?
        .build()
}

pub fn emit_key(virtual_device: &mut VirtualDevice, key_code: u16, key_value: i32) {
    let event = InputEvent::new(EventType::KEY, key_code, key_value);

    virtual_device.emit(&[event]).unwrap();
}
