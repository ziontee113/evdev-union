#![allow(unused_macros)]
use std::fmt::Display;

use crate::{key_fragment::KeyFragment, union::Union};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub enum RuleInputType {
    Fragment(KeyFragment),
    Union(Union),
}

pub trait WrapInRuleInputType {
    fn wrap_me_in_rule_input_type_enum(&self) -> RuleInputType;
}

#[derive(Debug, Clone)]
pub struct RuleInput {
    components: Vec<RuleInputType>,
}

impl RuleInput {
    pub fn new(components: Vec<RuleInputType>) -> Self {
        Self { components }
    }
    pub fn components(&self) -> Vec<RuleInputType> {
        self.components.clone()
    }
}

impl Display for RuleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.components.iter();
        if let Some(component) = iter.next() {
            match component {
                RuleInputType::Fragment(value) => write!(f, "{value}")?,
                RuleInputType::Union(value) => write!(f, "{value}")?,
            }
        }
        for component in iter {
            match component {
                RuleInputType::Fragment(value) => write!(f, ", {value}")?,
                RuleInputType::Union(value) => write!(f, ", {value}")?,
            }
        }
        Ok(())
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
    fn can_turn_rule_input_to_string() {
        assert_eq!(
            "L1|D L1|F, R1|J",
            rule_input!(union!("L1|D", "L1|F"), fragment!("R1|J")).to_string()
        );

        assert_eq!(
            "L1|LEFTCTRL, R1|J",
            rule_input!(union!("L1|LEFTCTRL"), fragment!("R1|J")).to_string()
        );
    }
}
