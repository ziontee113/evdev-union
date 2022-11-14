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
}

#[cfg(test)]
mod key_fragment_test {
    use evdev::Key;

    use super::*;

    #[test]
    fn create_new_key_fragment() {
        let _ = KeyFragment::new("L1", Key::KEY_A.code());
    }
}
