use crate::sys::{FileFlags, FileID};

use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub fn read(fid: FileID, buf: &mut [u8]) -> u64 {
    let raw_handle = fid as RawHandle;
    let mut file = unsafe { File::from_raw_handle(raw_handle) };

    let n = file
        .read(buf)
        .map_err(|err| panic!("Failed to read from file: {fid}: {err}."))
        .unwrap();

    std::mem::forget(file);
    n as u64
}

pub fn write(fid: FileID, bytes_to_write: &[u8]) {
    let raw_handle = fid as RawHandle;
    let mut file = unsafe { File::from_raw_handle(raw_handle) };

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

    let raw_handle = file.as_raw_handle();
    let fid = raw_handle as FileID;

    std::mem::forget(file);
    fid
}

pub fn close(fid: FileID) {
    let raw_handle = fid as RawHandle;
    let mut file = unsafe { File::from_raw_handle(raw_handle) };
    drop(file);
}
