use crate::{key_fragment::KeyFragment, union::Union};

#[allow(clippy::module_name_repetitions)]
pub enum RuleInputType {
    Fragment(KeyFragment),
    Union(Union),
}

pub struct RuleInput {
    components: Vec<RuleInputType>,
}

impl RuleInput {
    pub fn new(components: Vec<RuleInputType>) -> Self {
        Self { components }
    }
}

#[cfg(test)]
mod test_rule_input {
    use super::*;
    use crate::union;

    #[test]
    fn can_create_rule_input() {
        let union = union!("L1|D", "L1|F");
        let j_fragment = KeyFragment::from_str("R1|J");

        let _rule_input = RuleInput::new(vec![
            RuleInputType::Union(union),
            RuleInputType::Fragment(j_fragment),
        ]);
    }
}
