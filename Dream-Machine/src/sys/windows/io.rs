use crate::sys::{FileID, OpenFlags, BADFID, STDERR, STDIN, STDOUT};

use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    mem::ManuallyDrop,
};

use winapi::{
    shared::minwindef::DWORD,
    um::{
        winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
        GetStdHandle,
    },
};

fn to_raw_handle(fid: FileID) -> RawHandle {
    match fid {
        BADFID => 0 as RawHandle,
        STDIN => GetStdHandle(STD_INPUT_HANDLE),
        STDOUT => GetStdHandle(STD_OUTPUT_HANDLE),
        STDERR => GetStdHandle(STD_ERROR_HANDLE),
        _ => fid as RawHandle,
    }
}

fn from_raw_handle(raw_handle: RawHandle) -> FileID {
    let in_handle = GetStdHandle(STD_INPUT_HANDLE);
    let out_handle = GetStdHandle(STD_OUTPUT_HANDLE);
    let err_handle = GetStdHandle(STD_ERROR_HANDLE);

    if raw_handle == in_handle {
        STDIN
    } else if raw_handle == out_handle {
        STDOUT
    } else if raw_handle == err_handle {
        STDERR
    } else {
        raw_handle as FileID
    }
}

pub fn read(fid: FileID, buf: &mut [u8]) -> u64 {
    let raw_handle = to_raw_handle(fid);
    let mut file = ManuallyDrop::new(unsafe { File::from_raw_handle(raw_handle) });

    let n = file
        .read(buf)
        .map_err(|err| panic!("Failed to read from file: {fid}: {err}."))
        .unwrap();

    n as u64
}

pub fn write(fid: FileID, bytes_to_write: &[u8]) {
    let raw_handle = to_raw_handle(fid);
    let mut file = ManuallyDrop::new(unsafe { File::from_raw_handle(raw_handle) });

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

    let raw_handle = file.as_raw_handle();
    let fid = from_raw_handle(raw_handle);

    fid
}

pub fn close(fid: FileID) {
    let raw_handle = to_raw_handle(fid);
    let file = unsafe { File::from_raw_handle(raw_handle) };
    drop(file);
}
