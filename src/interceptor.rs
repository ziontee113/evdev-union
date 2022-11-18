use std::{collections::HashMap, sync::Arc, sync::Mutex, thread, time::SystemTime};

use crate::{
    arc_mu,
    devices::{self, physical::InputEventKindCheck},
    input::{raw_input_collector::RawInputCollector, raw_input_processor::RawInputProcessor},
    ruleset::RuleSet,
    utils,
};

pub fn start(alias_map: &HashMap<&str, &str>) {
    let mut handles = vec![];
    let collector = arc_mu!(RawInputCollector::new());
    let ruleset = arc_mu!(utils::create_mock_ruleset());

    for (device_alias, device_path) in alias_map {
        let handle = intercept_device(device_alias, device_path, &collector, &ruleset);
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn intercept_device(
    device_alias: &str,
    device_path: &str,
    collector: &Arc<Mutex<RawInputCollector>>,
    ruleset: &Arc<Mutex<RuleSet>>,
) -> std::thread::JoinHandle<()> {
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

    let device_alias = device_alias.to_string();
    // let device_path = device_path.to_string();
    let collector = Arc::clone(collector);
    let ruleset = Arc::clone(ruleset);

    thread::spawn(move || loop {
        for ev in d.fetch_events().unwrap() {
            if ev.is_type_key() {
                let mut collector = collector.lock().unwrap();
                collector.collect_event(&device_alias, ev.value(), ev.code(), SystemTime::now());

                let ruleset = ruleset.lock().unwrap();
                RawInputProcessor::process(&collector, &ruleset);
            }
        }
    })
}
