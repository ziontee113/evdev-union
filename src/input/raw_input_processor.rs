use super::raw_input_collector::RawInputCollector;
use crate::ruleset::RuleSet;

pub struct RawInputProcessor;

impl RawInputProcessor {
    pub fn process(collector: &RawInputCollector, ruleset: &RuleSet) {
        let union_interval_hash_map = ruleset.get_union_interval_hash_map();
        dbg!(union_interval_hash_map);

        dbg!(collector.get_fragments_before_release());
        dbg!(collector.get_fragments_after_press());

        println!("----------------------------------------");
    }
}

#[cfg(test)]
mod input_processor_test {
    use super::*;
    use crate::{
        input::raw_input_collector::RawInputCollector,
        utils::{self, millis_from_epoch},
    };
    use evdev::Key;

    #[test]
    fn can_process_raw_input() {
        let mut collector = RawInputCollector::new();
        let ruleset = utils::create_mock_ruleset();

        collector.collect_event("L1", 1, Key::KEY_LEFTCTRL.code(), millis_from_epoch(0));
        RawInputProcessor::process(&collector, &ruleset);

        collector.collect_event("L1", 2, Key::KEY_LEFTCTRL.code(), millis_from_epoch(20));
        RawInputProcessor::process(&collector, &ruleset);

        collector.collect_event("R1", 1, Key::KEY_J.code(), millis_from_epoch(40));
        RawInputProcessor::process(&collector, &ruleset);

        collector.collect_event("R1", 0, Key::KEY_J.code(), millis_from_epoch(50));
        RawInputProcessor::process(&collector, &ruleset);

        collector.collect_event("L1", 0, Key::KEY_LEFTCTRL.code(), millis_from_epoch(80));
        RawInputProcessor::process(&collector, &ruleset);
    }
}
