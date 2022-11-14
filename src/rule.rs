use crate::{key_fragment::KeyFragment, union::Union};

struct Rule {
    input: RuleInput,
    output: RuleOutput,
}

impl Rule {
    pub fn new(input: RuleInput, output: RuleOutput) -> Self {
        Self { input, output }
    }
}

enum RuleInputType {
    Fragment(KeyFragment),
    Union(Union),
}

struct RuleInput {
    components: Vec<RuleInputType>,
}

impl RuleInput {
    pub fn new(components: Vec<RuleInputType>) -> Self {
        Self { components }
    }
}

struct OutputCommand {
    command: String,
}

impl OutputCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
}

struct OutputKeySequence {
    sequence: Vec<u16>,
}

impl OutputKeySequence {
    pub fn new(sequence: Vec<u16>) -> Self {
        Self { sequence }
    }
}

enum RuleOutput {
    Command(OutputCommand),
    KeySequence(OutputKeySequence),
}

#[cfg(test)]
mod rule_test {
    use evdev::Key;

    use super::*;

    fn create_key_sequence_output(sequence: Vec<u16>) -> RuleOutput {
        RuleOutput::KeySequence(OutputKeySequence::new(sequence))
    }
    fn create_command_output(command: &str) -> RuleOutput {
        let output_command = OutputCommand::new(command);
        RuleOutput::Command(output_command)
    }

    #[test]
    fn create_rule() {
        // create input_1
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        let l1_df_union = Union::new(vec![l1_d_fragment, l1_f_fragment], 30);

        let r1_j_fragment = KeyFragment::new("R1", Key::KEY_J.code());

        let rule_components = vec![
            RuleInputType::Union(l1_df_union),
            RuleInputType::Fragment(r1_j_fragment),
        ];
        let input_1 = RuleInput::new(rule_components);

        // create input_2
        let l1_leftctrl_fragment = KeyFragment::new("L1", Key::KEY_LEFTCTRL.code());
        let r1_j_fragment = KeyFragment::new("R1", Key::KEY_J.code());

        let rule_components = vec![
            RuleInputType::Fragment(l1_leftctrl_fragment),
            RuleInputType::Fragment(r1_j_fragment),
        ];
        let input_2 = RuleInput::new(rule_components);

        // create rules from inputs and outputs
        let command_output = create_command_output("firefox");
        let key_sequence_output = create_key_sequence_output(vec![Key::KEY_DOWN.code()]);

        let _rule_1 = Rule::new(input_1, command_output);
        let _rule_2 = Rule::new(input_2, key_sequence_output);
    }
}
