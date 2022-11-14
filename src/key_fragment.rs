pub struct KeyFragment {
    device_alias: String,
    key_code: u16,
}

impl KeyFragment {
    pub fn new(deivice_alias: &str, key_code: u16) -> Self {
        Self {
            device_alias: deivice_alias.to_string(),
            key_code,
        }
    }
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
    fn get_device_alias() {
        let l1_a_fragment = create_l1_a_fragment();
        assert_eq!(l1_a_fragment.get_device_alias(), "L1".to_string());
    }

    #[test]
    fn get_key_code() {
        let l1_a_fragment = create_l1_a_fragment();
        assert_eq!(l1_a_fragment.get_key_code(), Key::KEY_A.code());
    }
}
