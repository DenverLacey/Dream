use crate::sys::FileID;
use std::{
    fs::File,
    io::Write,
    os::{fd::RawFd, unix::io::FromRawFd},
};

pub fn write(fid: FileID, bytes_to_write: &[u8]) {
    let raw_fd = fid as RawFd;
    let mut file = unsafe { File::from_raw_fd(raw_fd) };

    match file.write(bytes_to_write) {
        Ok(bytes_written) => {
            if bytes_written != bytes_to_write.len() {
                panic!("Failed to write all bytes to file: {fid}");
            }
        }
        Err(err) => panic!("Failed to write to file: {fid}: {err}"),
    }

    std::mem::forget(file);
}

pub fn close(fid: FileID) {
    let raw_fd = fid as RawFd;
    let file = unsafe { File::from_raw_fd(raw_fd) };
    drop(file);
}
