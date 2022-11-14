use crate::key_fragment::KeyFragment;

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
}

#[cfg(test)]
mod union_test {
    use super::*;
    use crate::key_fragment::KeyFragment;
    use evdev::Key;

    fn create_l1_df_union() -> Union {
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        Union::new(vec![l1_d_fragment, l1_f_fragment], 30)
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
}
