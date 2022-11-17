#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod devices;
mod input;
mod interceptor;
mod key_fragment;
mod keycodes;
mod rule;
mod ruleset;
mod union;
mod utils;

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

fn main() {
    // let virtual_device = arc_mu!(devices::_virtual::new());
    let alias_map = create_mock_alias_dictionary();

    interceptor::start(&alias_map);
}

fn create_mock_alias_dictionary() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
    ])
}
