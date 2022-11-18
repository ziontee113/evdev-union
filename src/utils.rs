use std::time::{Duration, SystemTime};

#[macro_export]
macro_rules! arc_mu {
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    };
}

pub fn millis_from_epoch(milis: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_millis(milis)
}
