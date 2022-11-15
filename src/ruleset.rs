use crate::rule::Rule;

pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

#[cfg(test)]
mod rule_set_test {

    use evdev::Key;

    use crate::key_fragment::KeyFragment;
    use crate::rule::rule_input::{RuleInput, WrapInRuleInputType};
    use crate::rule::rule_output::{OutputKeySequence, WrapMeInRuleOutput};
    use crate::{fragment, rule};
    use crate::{rule_input, rule_output_sequence};

    use super::*;

    #[test]
    fn can_create_ruleset() {
        let first_rule = rule!(
            rule_input!(fragment!("L1|R"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_F12.code(), Key::KEY_J.code()).to_output()
        );
        let second_rule = rule!(
            rule_input!(fragment!("L1|LEFTCTRL"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_DOWN.code()).to_output()
        );

        let _ruleset = RuleSet::new(vec![first_rule, second_rule]);
    }
}
