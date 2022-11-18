use super::raw_input_fragment::RawInputFragment;
use std::time::SystemTime;

#[derive(Debug)]
pub struct RawInputCollector {
    fragments_after: Vec<RawInputFragment>,
    fragments_before: Vec<RawInputFragment>,
}

impl RawInputCollector {
    fn add_fragment(&mut self, device_alias: &str, code: u16, time: SystemTime) {
        let new_fragment = RawInputFragment::new(device_alias, code, time);
        self.fragments_after.push(new_fragment);
    }
    fn remove_fragment(&mut self, device_alias: &str, code: u16) {
        let i = self
            .fragments_after
            .iter()
            .position(|m| m.get_device_alias() == device_alias && m.get_code() == code);
        if let Some(i) = i {
            self.fragments_after.remove(i);
        }
    }
}

impl RawInputCollector {
    pub fn new() -> Self {
        Self {
            fragments_after: vec![],
            fragments_before: vec![],
        }
    }
    pub fn collect_event(
        &mut self,
        device_alias: &str,
        value: i32,
        code: u16,
        time_pressed: SystemTime,
    ) {
        self.fragments_before = self.fragments_after.clone();

        match value {
            0 => {
                self.remove_fragment(device_alias, code);
            }
            1 => {
                self.add_fragment(device_alias, code, time_pressed);
            }
            _ => (),
        }
    }
    pub fn get_fragments_after_press(&self) -> &Vec<RawInputFragment> {
        &self.fragments_after
    }
    pub fn get_fragments_before_release(&self) -> &Vec<RawInputFragment> {
        &self.fragments_before
    }
}

#[cfg(test)]
mod raw_input_collector_tests {
    use super::*;
    use crate::utils::millis_from_epoch;
    use evdev::Key;

    #[test]
    fn can_collect_events() {
        let mut collector = RawInputCollector::new();

        // press L1|LEFTCTRL
        collector.collect_event("L1", 1, Key::KEY_LEFTCTRL.code(), millis_from_epoch(0));
        assert_eq!(collector.fragments_before.len(), 0);
        assert_eq!(collector.fragments_after.len(), 1);
        assert_eq!(
            collector.fragments_after.get(0).unwrap().get_code(),
            Key::KEY_LEFTCTRL.code()
        );
        assert_eq!(
            collector.fragments_after.get(0).unwrap().get_device_alias(),
            "L1"
        );

        // holding down L1|LEFTCTRL
        collector.collect_event("L1", 2, Key::KEY_LEFTCTRL.code(), millis_from_epoch(20));
        assert_eq!(collector.fragments_before.len(), 1);
        assert_eq!(collector.fragments_after.len(), 1);

        // press R1|J
        collector.collect_event("R1", 1, Key::KEY_J.code(), millis_from_epoch(40));
        assert_eq!(collector.fragments_before.len(), 1);
        assert_eq!(collector.fragments_after.len(), 2);
        assert_eq!(
            collector.fragments_after.get(1).unwrap().get_code(),
            Key::KEY_J.code()
        );
        assert_eq!(
            collector.fragments_after.get(1).unwrap().get_device_alias(),
            "R1"
        );

        // release R1|J
        collector.collect_event("R1", 0, Key::KEY_J.code(), millis_from_epoch(50));
        assert_eq!(collector.fragments_before.len(), 2);
        assert_eq!(collector.fragments_after.len(), 1);

        // release L1|LEFTCTRL
        collector.collect_event("L1", 0, Key::KEY_LEFTCTRL.code(), millis_from_epoch(80));
        assert_eq!(collector.fragments_before.len(), 1);
        assert_eq!(collector.fragments_after.len(), 0);
    }
}
