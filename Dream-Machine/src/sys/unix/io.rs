use crate::sys::{FileID, OpenFlags, STDERR};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    mem::ManuallyDrop,
    os::{
        fd::{AsRawFd, RawFd},
        unix::io::FromRawFd,
    },
};

fn to_raw_fd(fid: FileID) -> RawFd {
    if fid <= STDERR {
        fid.wrapping_sub(1) as RawFd
    } else {
        fid as RawFd
    }
}

fn from_raw_fd(fd: RawFd) -> FileID {
    if fd <= 2 {
        (fd + 1) as FileID
    } else {
        fd as FileID
    }
}

pub fn read(fid: FileID, buf: &mut [u8]) -> u64 {
    let raw_fd = to_raw_fd(fid);
    let mut file = ManuallyDrop::new(unsafe { File::from_raw_fd(raw_fd) });

    let n = file
        .read(buf)
        .map_err(|err| panic!("Failed to read from file: {fid}: {err}."))
        .unwrap();

    n as u64
}

pub fn write(fid: FileID, bytes_to_write: &[u8]) {
    let raw_fd = to_raw_fd(fid);
    let mut file = ManuallyDrop::new(unsafe { File::from_raw_fd(raw_fd) });

    match file.write(bytes_to_write) {
        Ok(bytes_written) => {
            if bytes_written != bytes_to_write.len() {
                panic!("Failed to write all bytes to file: {fid}");
            }
        }
        Err(err) => panic!("Failed to write to file: {fid}: {err}"),
    }
}

pub fn open(path: &str, flags: OpenFlags) -> FileID {
    let file = ManuallyDrop::new(
        OpenOptions::new()
            .create_new((flags & OpenFlags::CreateNew) == OpenFlags::CreateNew)
            .create((flags & OpenFlags::Create) == OpenFlags::Create)
            .read((flags & OpenFlags::Read) == OpenFlags::Read)
            .write((flags & OpenFlags::Write) == OpenFlags::Write)
            .append((flags & OpenFlags::Append) == OpenFlags::Append)
            .truncate((flags & OpenFlags::Truncate) == OpenFlags::Truncate)
            .open(path)
            .map_err(|err| panic!("Cannot open file: {path}: {err}."))
            .unwrap(),
    );

    let raw_fd = file.as_raw_fd();
    let fid = from_raw_fd(raw_fd);

    fid
}

pub fn close(fid: FileID) {
    let raw_fd = to_raw_fd(fid);
    let file = unsafe { File::from_raw_fd(raw_fd) };
    drop(file);
}
