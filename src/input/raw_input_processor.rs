use super::raw_input_fragment::RawInputFragment;

pub struct RawInputProcessor;

impl RawInputProcessor {
    pub fn process(before: &[RawInputFragment], after: &[RawInputFragment]) {
        dbg!(before);
        dbg!(after);
        println!("----------------------------------------");
    }
}

#[cfg(test)]
mod input_processor_test {
    // use super::*;

    use std::time::{Duration, SystemTime};

    fn millis_from_unix_epoch(milis: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
    }

    #[test]
    fn millis_from_unix_epoch_test() {
        let time = millis_from_unix_epoch(7);

        assert_eq!(
            time.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            7
        );
    }
}
