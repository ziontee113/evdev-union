use std::{cmp, collections::HashMap};

use crate::rule::{rule_input::RuleInputType, Rule};

pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
    fn generate_union_hash_map(&self) -> HashMap<String, u32> {
        let mut map = HashMap::new();

        self.rules.iter().for_each(|rule| {
            rule.input().components().into_iter().for_each(|component| {
                if let RuleInputType::Union(union) = component {
                    let key = union.to_string();
                    let interval = union.get_interval_limit();

                    map.entry(key)
                        .and_modify(|cur_interval| {
                            *cur_interval = cmp::max(*cur_interval, interval);
                        })
                        .or_insert(interval);
                }
            });
        });

        map
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
    // TODO: look into Rust auto import, or alternatives, this is a mess!

    #[test]
    fn can_create_union_hashmap() {
        let first_rule = rule!(
            rule_input!(union!("L1|D", "L1|F"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_A.code()).to_output()
        );
        let second_rule = rule!(
            rule_input!(fragment!("L1|LEFTCTRL"), fragment!("R1|J")),
            rule_output_sequence!(Key::KEY_DOWN.code()).to_output()
        );

        let ruleset = RuleSet::new(vec![first_rule, second_rule]);
        let union_hashmap = ruleset.generate_union_hash_map();

        dbg!(union_hashmap);
    }
}
