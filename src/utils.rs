#[macro_export]
macro_rules! arc_mu {
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    };
}
