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

fn main() {
    // let virtual_device = arc_mu!(devices::_virtual::new());
    let alias_map = utils::create_mock_alias_dictionary();

    interceptor::start(&alias_map);
}
