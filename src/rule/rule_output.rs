pub struct OutputCommand {
    command: String,
}

impl OutputCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
}

pub struct OutputKeySequence {
    sequence: Vec<u16>,
}

impl OutputKeySequence {
    pub fn new(sequence: Vec<u16>) -> Self {
        Self { sequence }
    }
}

pub enum RuleOutput {
    Command(OutputCommand),
    KeySequence(OutputKeySequence),
}

#[cfg(test)]
mod test_rule_output {
    use evdev::Key;

    use super::*;

    #[test]
    fn can_create_rule_output_command() {
        let output_command = OutputCommand::new("firefox");
        let _ = RuleOutput::Command(output_command);
    }

    #[test]
    fn can_create_rule_output_key_sequence() {
        let output_key_sequence = OutputKeySequence::new(vec![Key::KEY_DOWN.code()]);
        let _ = RuleOutput::KeySequence(output_key_sequence);
    }
}
