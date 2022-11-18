use super::raw_input_fragment::RawInputFragment;

pub struct RawInputProcessor;

impl RawInputProcessor {
    pub fn process(before: &[RawInputFragment], after: &[RawInputFragment]) {
        dbg!(before);
        dbg!(after);
        println!("----------------------------------------");
    }
}

#[cfg(test)]
mod input_processor_test {
    use super::*;
    use crate::{input::raw_input_collector::RawInputCollector, utils::millis_from_epoch};
    use evdev::Key;

    #[test]
    fn can_process_raw_input() {
        let mut collector = RawInputCollector::new();

        collector.collect_event("L1", 1, Key::KEY_LEFTCTRL.code(), millis_from_epoch(0));
        RawInputProcessor::process(
            collector.get_fragments_before_release(),
            collector.get_fragments_after_press(),
        );

        collector.collect_event("L1", 2, Key::KEY_LEFTCTRL.code(), millis_from_epoch(20));
        RawInputProcessor::process(
            collector.get_fragments_before_release(),
            collector.get_fragments_after_press(),
        );

        collector.collect_event("R1", 1, Key::KEY_J.code(), millis_from_epoch(40));
        RawInputProcessor::process(
            collector.get_fragments_before_release(),
            collector.get_fragments_after_press(),
        );

        collector.collect_event("R1", 0, Key::KEY_J.code(), millis_from_epoch(50));
        RawInputProcessor::process(
            collector.get_fragments_before_release(),
            collector.get_fragments_after_press(),
        );

        collector.collect_event("L1", 0, Key::KEY_LEFTCTRL.code(), millis_from_epoch(80));
        RawInputProcessor::process(
            collector.get_fragments_before_release(),
            collector.get_fragments_after_press(),
        );
    }
}
