#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        let _ = crate::utils::logger::log_error(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        let _ = crate::utils::logger::log_debug(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        let _ = crate::utils::logger::log_message(&format!($($arg)*));
    }
}
