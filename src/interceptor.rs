use std::{collections::HashMap, thread};

use crate::devices::{self, physical::InputEventKindCheck};

pub fn start(alias_map: &HashMap<&str, &str>) {
    let mut handles = vec![];

    for (device_alias, device_path) in alias_map {
        let handle = intercept_device(device_alias, device_path);
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn intercept_device(device_alias: &str, device_path: &str) -> std::thread::JoinHandle<()> {
    let mut d = devices::physical::from_path(device_path);
    match d.grab() {
        Ok(_) => println!("Grabbed {} {} SUCCESSFULLY", device_alias, device_path),
        Err(err) => {
            println!(
                "FAILED TO GRAB {} {},\n{},\n------------------",
                device_alias, device_path, err
            );
        }
    }

    thread::spawn(move || loop {
        for ev in d.fetch_events().unwrap() {
            if ev.is_type_key() {
                dbg!(ev);
            }
        }
    })
}
