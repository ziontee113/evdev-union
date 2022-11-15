#![allow(unused_macros)]
use crate::{key_fragment::KeyFragment, union::Union};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum RuleInputType {
    Fragment(KeyFragment),
    Union(Union),
}

pub trait WrapInRuleInputType {
    fn wrap_me_in_rule_input_type_enum(&self) -> RuleInputType;
}

#[derive(Debug)]
pub struct RuleInput {
    components: Vec<RuleInputType>,
}

impl RuleInput {
    pub fn new(components: Vec<RuleInputType>) -> Self {
        Self { components }
    }
}

#[macro_export]
macro_rules! rule_input {
    ($($a:expr), *) => {
        RuleInput::new(vec![ $($a.wrap_me_in_rule_input_type_enum(),)* ])
    };
}

#[cfg(test)]
mod test_rule_input {
    use super::*;
    use crate::{fragment, union};

    #[test]
    fn can_create_rule_input() {
        let union = union!("L1|D", "L1|F");
        let j_fragment = fragment!("R1|J");

        let _rule_input = RuleInput::new(vec![
            RuleInputType::Union(union),
            RuleInputType::Fragment(j_fragment),
        ]);
    }

    #[test]
    fn rule_input_macro() {
        let union = union!("L1|D", "L1|F");
        let j_fragment = fragment!("R1|J");

        let rule_input = rule_input!(union, j_fragment);
        dbg!(rule_input);

        let rule_input_2 = rule_input!(union!("L1|D", "L1|F", "L1|S"), fragment!("R1|K"));
        dbg!(rule_input_2);
    }
}
