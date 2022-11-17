use crate::{
    keycodes::KEY_COLLECTION,
    rule::rule_input::{RuleInputType, WrapInRuleInputType},
};
use evdev::Key;
use std::{fmt::Display, str::FromStr};

#[allow(unused_macros)]
#[macro_export]
macro_rules! fragment {
    ($a:expr) => {
        KeyFragment::from_str($a)
    };
}

#[derive(Debug, Clone)]
pub struct KeyFragment {
    device_alias: String,
    key_code: u16,
}

impl WrapInRuleInputType for KeyFragment {
    fn wrap_me_in_rule_input_type_enum(&self) -> RuleInputType {
        RuleInputType::Fragment(self.clone())
    }
}

impl KeyFragment {
    pub fn new(deivice_alias: &str, key_code: u16) -> Self {
        Self {
            device_alias: deivice_alias.to_string(),
            key_code,
        }
    }
    pub fn from_str(str: &str) -> Self {
        let mut split = str.split('|');
        let device_alias = split.next().unwrap().to_string();
        let mut key = split.next().unwrap().to_string().to_uppercase();
        key = format!("KEY_{}", key);
        let key_code = Key::from_str(&key).unwrap().code();

        Self {
            device_alias,
            key_code,
        }
    }
}

impl Display for KeyFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}",
            self.device_alias,
            (*KEY_COLLECTION.get(usize::from(self.key_code)).unwrap())
        )
    }
}

impl KeyFragment {
    pub fn get_device_alias(&self) -> String {
        self.device_alias.to_string()
    }
    pub fn get_key_code(&self) -> u16 {
        self.key_code
    }
}

#[cfg(test)]
mod key_fragment_test {
    use super::*;
    use evdev::Key;

    fn create_l1_a_fragment() -> KeyFragment {
        KeyFragment::new("L1", Key::KEY_A.code())
    }

    #[test]
    fn can_get_device_alias() {
        assert_eq!(create_l1_a_fragment().get_device_alias(), "L1".to_string());
    }

    #[test]
    fn can_get_key_code() {
        assert_eq!(create_l1_a_fragment().get_key_code(), Key::KEY_A.code());
    }

    #[test]
    fn can_create_fragment_from_str() {
        let fragment = KeyFragment::from_str("L1|D");
        assert_eq!("L1", fragment.get_device_alias());
        assert_eq!(Key::KEY_D.code(), fragment.get_key_code());
    }

    #[test]
    fn can_create_fragment_from_macro() {
        let fragment = fragment!("L1|D");
        assert_eq!("L1", fragment.get_device_alias());
        assert_eq!(Key::KEY_D.code(), fragment.get_key_code());
    }

    #[test]
    fn can_turn_fragment_to_string() {
        let result = fragment!("L1|D").to_string();
        assert_eq!("L1|D".to_string(), result);
    }
}
