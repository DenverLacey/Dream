#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "unix")]
pub use crate::sys::unix::*;

#[cfg(target_family = "windows")]
pub use crate::sys::windows::*;

pub type FileID = u64;
