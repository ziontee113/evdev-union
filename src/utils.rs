use evdev::Key;

use crate::key_fragment::KeyFragment;
use crate::rule::rule_input::{RuleInput, WrapInRuleInputType};
use crate::rule::rule_output::{OutputCommand, OutputKeySequence, WrapMeInRuleOutput};
use crate::union::Union;
use crate::{
    fragment, rule, rule::Rule, rule_input, rule_output_cmd, rule_output_sequence,
    ruleset::RuleSet, union,
};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[macro_export]
macro_rules! arc_mu {
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    };
}

pub fn millis_from_epoch(milis: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
}

pub fn create_mock_ruleset() -> RuleSet {
    let first_rule = rule!(
        rule_input!(union!("L1|D" "L1|F"), fragment!("R1|J")),
        rule_output_cmd!("firefox").to_output()
    );
    let second_rule = rule!(
        rule_input!(fragment!("L1|LEFTCTRL"), fragment!("R1|J")),
        rule_output_sequence!(Key::KEY_DOWN.code()).to_output()
    );
    let third_rule = rule!(
        rule_input!(union!("L1|SPACE" "L1|V"), fragment!("R1|K")),
        rule_output_sequence!(Key::KEY_UP.code()).to_output()
    );
    let fourth_rule = rule!(
        rule_input!(union!("L1|F" "L1|D" => 40), fragment!("R1|K")),
        rule_output_sequence!(Key::KEY_UP.code()).to_output()
    );

    RuleSet::new(
        "Base RuleSet",
        vec![first_rule, second_rule, third_rule, fourth_rule],
    )
}

pub fn create_mock_alias_dictionary() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("L1", "usb-0000:00:1d.0-1.5.1.4/input0"),
        ("R1", "usb-0000:00:1d.0-1.5.2/input0"),
    ])
}
