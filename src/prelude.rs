pub use std::path::PathBuf;

use eyre::Error;

// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
#[allow(dead_code)]
pub struct W<T>(pub T);

pub const DATE_SEP: &str = "_";
pub const DATE_TIME_SEP: &str = " ";
pub const TIME_SEP: &str = ".";
pub const AM_PM_SEP: &str = "";

/// Fallback stamp when media metadata has no creation date.
/// Prefer logging at the call site; this string must stay parseable by `Stamp`.
pub const EPOCH_STAMP: &str = "01/01/1970 00:00 12:00 AM";

#[cfg(debug_assertions)]
pub fn default_data_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("data")
}

/// Release builds resolve `./data` next to the executable.
#[cfg(not(debug_assertions))]
pub fn default_data_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("data")))
        .unwrap_or_else(|| PathBuf::from("data"))
}
