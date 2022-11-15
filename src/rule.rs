mod rule_input;
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
        rule_input::{RuleInput, RuleInputType},
        rule_output::{OutputKeySequence, RuleOutput},
        Rule,
    };
    use crate::{fragment, key_fragment::KeyFragment, union, union::Union};
    use evdev::Key;

    #[test]
    fn can_create_rule() {
        let union = union!("L1|D", "L1|F" => 25);
        let fragment = fragment!("R1|J");
        let input = RuleInput::new(vec![
            RuleInputType::Union(union),
            RuleInputType::Fragment(fragment),
        ]);

        let output_key_sequence = OutputKeySequence::new(vec![Key::KEY_DOWN.code()]);
        let output = RuleOutput::KeySequence(output_key_sequence);

        let _rule = Rule::new(input, output);
    }
}
