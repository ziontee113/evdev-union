use super::raw_input_fragment::RawInputFragment;
use evdev::InputEvent;
use std::time::SystemTime;

#[derive(Debug)]
pub struct RawInputCollector {
    fragments: Vec<RawInputFragment>,
    before: Vec<RawInputFragment>,
    after: Vec<RawInputFragment>,
}

impl RawInputCollector {
    fn add_fragment(&mut self, device_alias: &str, code: u16, time: SystemTime) {
        let new_fragment = RawInputFragment::new(device_alias, code, time);
        self.fragments.push(new_fragment);
    }
    fn remove_fragment(&mut self, device_alias: &str, code: u16) {
        let i = self
            .fragments
            .iter()
            .position(|m| m.get_device_alias() == device_alias && m.get_code() == code);
        if let Some(i) = i {
            self.fragments.remove(i);
        }
    }
}

impl RawInputCollector {
    pub fn new() -> Self {
        Self {
            fragments: vec![],
            before: vec![],
            after: vec![],
        }
    }
    pub fn parse_input(&mut self, device_alias: &str, ev: InputEvent) {
        let value = ev.value();
        let code = ev.code();
        self.before = self.fragments.clone();

        match value {
            0 => {
                self.remove_fragment(device_alias, code);
            }
            1 => {
                self.add_fragment(device_alias, code, SystemTime::now());
            }
            _ => (),
        }

        self.after = self.fragments.clone();
    }

    pub fn print(&self) {
        let mut print_str = String::from("Before: ");
        for fragment in &self.before {
            print_str = format!(
                "{print_str} {}-{}, {}",
                fragment.get_device_alias(),
                fragment.get_code(),
                fragment.time_since_pressed()
            );
        }
        println!("{print_str}");

        let mut print_str = String::from("After: ");
        for fragment in &self.after {
            print_str = format!(
                "{print_str} {}-{}, {}",
                fragment.get_device_alias(),
                fragment.get_code(),
                fragment.time_since_pressed()
            );
        }
        println!("{print_str}");

        println!("------------------------------------");
    }
}
