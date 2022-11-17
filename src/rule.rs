#![allow(unused_macros)]

pub(crate) mod rule_input;
pub(crate) mod rule_output;
use self::{rule_input::RuleInput, rule_output::RuleOutput};

#[derive(Debug)]
pub struct Rule {
    input: RuleInput,
    output: RuleOutput,
}

impl Rule {
    pub fn new(input: RuleInput, output: RuleOutput) -> Self {
        Self { input, output }
    }
    pub fn get_input(&self) -> RuleInput {
        self.input.clone()
    }
    pub fn get_output(&self) -> RuleOutput {
        self.output.clone()
    }
}

#[macro_export]
macro_rules! rule {
    ($input:expr, $output:expr) => {
        Rule::new($input, $output)
    };
}

#[cfg(test)]
mod rule_test {
    use super::{
        rule_output::{OutputKeySequence, WrapMeInRuleOutput},
        Rule,
    };
    use crate::{fragment, key_fragment::KeyFragment, rule_input, union, union::Union};
    use crate::{
        rule::rule_input::{RuleInput, WrapInRuleInputType},
        rule_output_sequence,
    };
    use evdev::Key;

    #[test]
    fn can_create_rule() {
        let input = rule_input!(union!("L1|D", "L1|F" => 25), fragment!("R1|J"));
        let output = rule_output_sequence!(Key::KEY_DOWN.code()).to_output();

        let _rule = Rule::new(input, output);
    }

    #[test]
    fn can_create_rule_with_macro() {
        let _rule = rule!(
            rule_input!(fragment!("L1|R"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_F12.code(), Key::KEY_J.code()).to_output()
        );
    }
}
