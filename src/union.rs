use crate::key_fragment::KeyFragment;

#[allow(unused_macros)]
#[macro_export]
macro_rules! union {
    ($($a:expr), *) => {
        Union::new(vec![ $( KeyFragment::from_str($a), )* ], 25)
    };
    ($($a:expr), * => $interval:expr) => {
        Union::new(vec![ $( KeyFragment::from_str($a), )* ], $interval)
    }
}

#[derive(Debug)]
pub struct Union {
    members: Vec<KeyFragment>,
    interval_limit: u32,
}

impl Union {
    pub fn new(members: Vec<KeyFragment>, interval_limit: u32) -> Self {
        Self {
            members,
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

#[cfg(test)]
mod union_test {
    use super::*;
    use crate::key_fragment::KeyFragment;
    use evdev::Key;

    fn create_l1_df_union() -> Union {
        let first_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let second_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        Union::new(vec![first_fragment, second_fragment], 30)
    }

    #[test]
    fn get_members() {
        let l1_df_union = create_l1_df_union();
        let members = l1_df_union.get_members();

        assert_eq!(members.get(0).unwrap().get_device_alias(), "L1".to_string());
        assert_eq!(members.get(0).unwrap().get_key_code(), Key::KEY_D.code());
        assert_eq!(members.get(1).unwrap().get_device_alias(), "L1".to_string());
        assert_eq!(members.get(1).unwrap().get_key_code(), Key::KEY_F.code());
    }

    #[test]
    fn get_interval_limit() {
        let l1_df_union = create_l1_df_union();
        let interval_limit = l1_df_union.get_interval_limit();
        assert_eq!(interval_limit, 30);
    }

    #[test]
    fn create_union_with_macro() {
        let union = union!("L1|D", "L1|F" => 25);
        let members = union.get_members();

        assert_eq!(members.get(0).unwrap().get_device_alias(), "L1");
        assert_eq!(members.get(1).unwrap().get_device_alias(), "L1");
        assert_eq!(members.get(0).unwrap().get_key_code(), Key::KEY_D.code());
        assert_eq!(members.get(1).unwrap().get_key_code(), Key::KEY_F.code());
    }

    #[test]
    fn set_interval_limit() {
        let target_interval = 40;
        let mut union = union!("L1|D", "L1|F" => target_interval);

        union.set_interval_limit(target_interval);
        assert_eq!(union.get_interval_limit(), target_interval);
    }
}
