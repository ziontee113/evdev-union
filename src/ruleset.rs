use std::{cmp, collections::HashMap};

use crate::rule::{rule_input::RuleInputType, Rule};

#[derive(Debug)]
pub struct RuleSet {
    name: String,
    rules: Vec<Rule>,
    union_hash_map: HashMap<String, u32>,
}

impl RuleSet {
    pub fn new(name: &str, rules: Vec<Rule>) -> Self {
        Self {
            name: name.to_string(),
            union_hash_map: RuleSet::create_union_hash_map(&rules),
            rules,
        }
    }
    fn get_union_hash_map(&self) -> &HashMap<String, u32> {
        &self.union_hash_map
    }
}

impl RuleSet {
    fn create_union_hash_map(rules: &Vec<Rule>) -> HashMap<String, u32> {
        let mut union_hash_map = HashMap::new();
        for rule in rules {
            rule.get_input()
                .components()
                .into_iter()
                .for_each(|component| {
                    if let RuleInputType::Union(union) = component {
                        let key = union.to_string();
                        let interval = union.get_interval_limit();

                        union_hash_map
                            .entry(key)
                            .and_modify(|cur_interval| {
                                *cur_interval = cmp::max(*cur_interval, interval);
                            })
                            .or_insert(interval);
                    }
                });
        }
        union_hash_map
    }
}

#[cfg(test)]
mod rule_set_test {
    use super::{Rule, RuleSet};
    use crate::key_fragment::KeyFragment;
    use crate::rule::rule_input::{RuleInput, WrapInRuleInputType};
    use crate::rule::rule_output::{OutputKeySequence, WrapMeInRuleOutput};
    use crate::union::Union;
    use crate::{fragment, rule, rule_input, rule_output_sequence, union};
    use evdev::Key;

    #[test]
    fn can_create_ruleset() {
        let first_rule = rule!(
            rule_input!(union!("L1|D", "L1|F"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_A.code()).to_output()
        );
        let second_rule = rule!(
            rule_input!(fragment!("L1|LEFTCTRL"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_DOWN.code()).to_output()
        );
        let third_rule = rule!(
            rule_input!(union!("L1|SPACE", "L1|V"), fragment!("R1|K")),
            rule_output_sequence!(Key::KEY_UP.code()).to_output()
        );
        let fourth_rule = rule!(
            rule_input!(union!("L1|D", "L1|F" => 40), fragment!("R1|K")),
            rule_output_sequence!(Key::KEY_UP.code()).to_output()
        );

        let ruleset = RuleSet::new(
            "Base RuleSet",
            vec![first_rule, second_rule, third_rule, fourth_rule],
        );

        let union_hashmap = ruleset.get_union_hash_map();
        assert!(union_hashmap.contains_key("L1|D L1|F"));
        assert!(union_hashmap.contains_key("L1|SPACE L1|V"));
        assert_eq!(*union_hashmap.get("L1|D L1|F").unwrap(), 40);

        dbg!(&ruleset);
    }
}
