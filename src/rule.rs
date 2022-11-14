struct Rule {
    input: RuleInput,
    // output: RuleOutput,
}

enum RuleInputType {
    Fragment,
    Union,
}

struct RuleInput {
    components: Vec<RuleInputType>,
}

impl RuleInput {
    pub fn new(components: Vec<RuleInputType>) -> Self {
        Self { components }
    }
}

#[cfg(test)]
mod rule_test {
    use super::*;

    #[test]
    fn create_rule_input() {
        let _ = RuleInput::new(vec![RuleInputType::Fragment]);
    }
}
