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
}

#[cfg(test)]
mod union_test {
    use evdev::Key;

    use crate::key_fragment::KeyFragment;

    use super::*;

    #[test]
    fn create_new_union() {
        let l1_d_fragment = KeyFragment::new("L1", Key::KEY_D.code());
        let l1_f_fragment = KeyFragment::new("L1", Key::KEY_F.code());
        let _ = Union::new(vec![l1_d_fragment, l1_f_fragment], 30);
    }
}
