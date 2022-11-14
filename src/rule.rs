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
    use crate::{key_fragment::KeyFragment, union::Union};
    use evdev::Key;

    #[test]
    fn can_create_rule() {
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        let l1_df_union = Union::new(vec![l1_d_fragment, l1_f_fragment], 30);
        let r1_j_fragment = KeyFragment::new("R1", Key::KEY_J.code());
        let input = RuleInput::new(vec![
            RuleInputType::Union(l1_df_union),
            RuleInputType::Fragment(r1_j_fragment),
        ]);

        let output_key_sequence = OutputKeySequence::new(vec![Key::KEY_DOWN.code()]);
        let output = RuleOutput::KeySequence(output_key_sequence);

        let _rule = Rule::new(input, output);
    }
}
