#![allow(unused_macros, clippy::module_name_repetitions)]

#[derive(Debug, Clone)]
pub struct OutputCommand {
    command: String,
}

impl OutputCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }
    pub fn command(&self) -> String {
        self.command.to_string()
    }
}

impl WrapMeInRuleOutput for OutputCommand {
    fn to_output(&self) -> RuleOutput {
        RuleOutput::Command(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct OutputKeySequence {
    sequence: Vec<u16>,
}

impl OutputKeySequence {
    pub fn new(sequence: Vec<u16>) -> Self {
        Self { sequence }
    }
    pub fn get_sequence(&self) -> &Vec<u16> {
        &self.sequence
    }
}

impl WrapMeInRuleOutput for OutputKeySequence {
    fn to_output(&self) -> RuleOutput {
        RuleOutput::KeySequence(self.clone())
    }
}

#[derive(Debug)]
pub enum RuleOutput {
    Command(OutputCommand),
    KeySequence(OutputKeySequence),
}

pub trait WrapMeInRuleOutput {
    fn to_output(&self) -> RuleOutput;
}

#[macro_export]
macro_rules! rule_output_cmd {
    ($a:expr) => {
        OutputCommand::new($a)
    };
}

#[macro_export]
macro_rules! rule_output_sequence {
    ($($a:expr), *) => {
        OutputKeySequence::new(vec![ $($a,)* ])
    };
}

#[cfg(test)]
mod test_rule_output {
    use super::*;
    use evdev::Key;

    #[test]
    fn can_create_rule_output_command() {
        let output_command = OutputCommand::new("firefox");
        let rule_output = RuleOutput::Command(output_command);

        if let RuleOutput::Command(output_command) = rule_output {
            println!("command = {}", output_command.command());
        }
    }

    #[test]
    fn can_create_rule_output_key_sequence() {
        let output_key_sequence = OutputKeySequence::new(vec![Key::KEY_DOWN.code()]);
        let rule_output = RuleOutput::KeySequence(output_key_sequence);

        if let RuleOutput::KeySequence(output_command) = rule_output {
            println!("key_sequence = {:?}", output_command.get_sequence());
        }
    }

    #[test]
    fn rule_output_cmd_macro() {
        let output_command = rule_output_cmd!("firefox");
        assert_eq!("firefox", output_command.command());
    }

    #[test]
    fn rule_output_sequence_macro() {
        let _output_sequence =
            rule_output_sequence!(Key::KEY_LEFTCTRL.code(), Key::KEY_DOWN.code());
    }

    #[test]
    fn to_output_methods() {
        let _cmd_rule_output = rule_output_cmd!("firefox").to_output();
        let _key_sequence_rule_output = rule_output_sequence!(Key::KEY_DOWN.code()).to_output();
    }
}
