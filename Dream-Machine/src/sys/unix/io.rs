use crate::sys::{FileFlags, FileID};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::{
        fd::{AsRawFd, RawFd},
        unix::io::FromRawFd,
    },
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

pub fn open(path: &str, flags: FileFlags) -> FileID {
    let file = OpenOptions::new()
        .create_new((flags & FileFlags::CreateNew) == FileFlags::CreateNew)
        .create((flags & FileFlags::Create) == FileFlags::Create)
        .read((flags & FileFlags::Read) == FileFlags::Read)
        .write((flags & FileFlags::Write) == FileFlags::Write)
        .append((flags & FileFlags::Append) == FileFlags::Append)
        .truncate((flags & FileFlags::Truncate) == FileFlags::Truncate)
        .open(path)
        .map_err(|err| panic!("Cannot open file: {path}: {err}."))
        .unwrap();

    let raw_fd = file.as_raw_fd();
    let fid = raw_fd as FileID;

    std::mem::forget(file);
    fid
}

pub fn close(fid: FileID) {
    let raw_fd = fid as RawFd;
    let file = unsafe { File::from_raw_fd(raw_fd) };
    drop(file);
}
