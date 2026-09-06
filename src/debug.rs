#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        cfg_select! {
            debug_assertions => {
                eprint!("DEBUG: ");
                eprintln!($($arg)*);
            },
            _ => {}
        }
    };
}
