use crate::{key_fragment::KeyFragment, union::Union};

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
    use evdev::Key;

    use super::*;

    #[test]
    fn can_create_rule_input() {
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());

        let l1_df_union = Union::new(vec![l1_d_fragment, l1_f_fragment], 30);
        let r1_j_fragment = KeyFragment::new("R1", Key::KEY_J.code());

        let _ = RuleInput::new(vec![
            RuleInputType::Union(l1_df_union),
            RuleInputType::Fragment(r1_j_fragment),
        ]);
    }
}
