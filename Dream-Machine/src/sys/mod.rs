#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "unix")]
pub use crate::sys::unix::*;

#[cfg(target_family = "windows")]
pub use crate::sys::windows::*;

pub type FileID = u64;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileFlags {
    None = 0x0,
    Read = 0x1,
    Write = 0x2,
    Append = 0x4,
    Truncate = 0x8,
    Create = 0x10,
    CreateNew = 0x20,
}

impl std::ops::BitOr for FileFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let s = self as u64;
        let rhs = rhs as u64;
        unsafe { std::mem::transmute(s | rhs) }
    }
}

impl std::ops::BitAnd for FileFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let s = self as u64;
        let rhs = rhs as u64;
        unsafe { std::mem::transmute(s & rhs) }
    }
}
