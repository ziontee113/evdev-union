pub(crate) mod rule_input;
mod rule_output;
use self::{rule_input::RuleInput, rule_output::RuleOutput};

struct Rule {
    input: RuleInput,
    output: RuleOutput,
}

impl Rule {
    pub fn new(input: RuleInput, output: RuleOutput) -> Self {
        Self { input, output }
    }
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
}
