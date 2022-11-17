use std::fmt::Display;

use crate::{
    key_fragment::KeyFragment,
    rule::rule_input::{RuleInputType, WrapInRuleInputType},
};

#[allow(unused_macros)]
#[macro_export]
macro_rules! union {
    ($($a:expr) *) => {
        Union::new(&mut [ $( KeyFragment::from_str($a), )* ], 25)
    };
    ($($a:expr) * => $interval:expr) => {
        Union::new(&mut [ $( KeyFragment::from_str($a), )* ], $interval)
    }
}

#[derive(Debug, Clone)]
pub struct Union {
    members: Vec<KeyFragment>,
    interval_limit: u32,
}

impl Union {
    fn sort_members(members: &mut [KeyFragment]) -> Vec<KeyFragment> {
        members.sort();
        members.to_owned()
    }
}

impl Union {
    pub fn new(members: &mut [KeyFragment], interval_limit: u32) -> Self {
        Self {
            members: Union::sort_members(members),
            interval_limit,
        }
    }
    pub fn get_members(&self) -> &Vec<KeyFragment> {
        &self.members
    }
    pub fn get_interval_limit(&self) -> u32 {
        self.interval_limit
    }
    pub fn set_interval_limit(&mut self, interval_limit: u32) {
        self.interval_limit = interval_limit;
    }
}

impl Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.members.iter();
        if let Some(member) = iter.next() {
            write!(f, "{member}")?;
        }
        for member in iter {
            write!(f, " {}", member)?;
        }
        Ok(())
    }
}

impl WrapInRuleInputType for Union {
    fn wrap_me_in_rule_input_type_enum(&self) -> RuleInputType {
        RuleInputType::Union(self.clone())
    }
}

#[cfg(test)]
mod union_test {
    use super::*;
    use crate::{fragment, key_fragment::KeyFragment};
    use evdev::Key;

    #[test]
    fn create_union_with_macro() {
        let union = union!("L1|D" "L1|F" => 25);
        let members = union.get_members();

        assert_eq!(members.get(0).unwrap().get_device_alias(), "L1");
        assert_eq!(members.get(1).unwrap().get_device_alias(), "L1");
        assert_eq!(members.get(0).unwrap().get_key_code(), Key::KEY_D.code());
        assert_eq!(members.get(1).unwrap().get_key_code(), Key::KEY_F.code());
    }

    #[test]
    fn can_get_interval_limit() {
        let union = union!("L1|D" "L1|F" => 30);
        assert_eq!(union.get_interval_limit(), 30);
    }

    #[test]
    fn can_set_interval_limit() {
        let target_interval = 40;
        let mut union = union!("L1|D" "L1|F" => target_interval);

        union.set_interval_limit(target_interval);
        assert_eq!(union.get_interval_limit(), target_interval);
    }

    #[test]
    fn can_sort_union_members() {
        let mut fragments = vec![
            fragment!("L1|F"),
            fragment!("L1|D"),
            fragment!("R1|K"),
            fragment!("R1|J"),
        ];
        let sorted = Union::sort_members(&mut fragments);

        assert_eq!(
            sorted,
            vec![
                fragment!("L1|D"),
                fragment!("L1|F"),
                fragment!("R1|J"),
                fragment!("R1|K"),
            ]
        );
    }
}
