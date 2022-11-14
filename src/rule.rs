use crate::{key_fragment::KeyFragment, union::Union};

struct Rule {
    input: RuleInput,
    // output: RuleOutput,
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

#[cfg(test)]
mod rule_test {
    use evdev::Key;

    use super::*;

    #[test]
    fn create_rule_input() {
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        let l1_df_union = Union::new(vec![l1_d_fragment, l1_f_fragment], 30);

        let r1_j_fragment = KeyFragment::new("R1", Key::KEY_J.code());

        let rule_components = vec![
            RuleInputType::Union(l1_df_union),
            RuleInputType::Fragment(r1_j_fragment),
        ];
        let _ = RuleInput::new(rule_components);
    }
}
