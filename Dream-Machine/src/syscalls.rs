// https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

use crate::sys;
use crate::vm::VM;

#[repr(u16)]
#[derive(Debug)]
pub enum Syscall {
    Read = 0,  // fid:FileID, buf:ptr, size:u64 -> num_bytes_read:u64
    Write = 1, // fid:FileID, buf:ptr, size:u64
    Open = 2,  // path_ptr:ptr, path_len:u64, flags:FileFlags -> fid:FileID
    Close = 3, // fid:FileID
}

pub fn syscall0(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        _ => panic!("Invalid syscall0: {syscall:?}"),
    }
}

pub fn syscall1(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        Syscall::Close => {
            let fid = vm.reg.sr[0] as sys::FileID;
            sys::io::close(fid);
        }
        _ => panic!("Unvalid syscall1: {syscall:?}"),
    }
}

pub fn syscall2(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        _ => panic!("Invalid syscall2: {syscall:?}"),
    }
}

pub fn syscall3(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        Syscall::Read => {
            let fid = vm.reg.sr[0] as sys::FileID;
            let buf = unsafe {
                std::slice::from_raw_parts_mut(vm.reg.sr[1] as *mut u8, vm.reg.sr[2] as usize)
            };
            vm.reg.srr = sys::io::read(fid, buf);
        }
        Syscall::Write => {
            let fid = vm.reg.sr[0] as sys::FileID;
            let bytes_to_write = unsafe {
                std::slice::from_raw_parts(vm.reg.sr[1] as *const u8, vm.reg.sr[2] as usize)
            };
            sys::io::write(fid, bytes_to_write);
        }
        Syscall::Open => {
            let path_ptr = vm.reg.sr[0] as *const u8;
            let path_len = vm.reg.sr[1] as usize;
            let flags = unsafe { std::mem::transmute(vm.reg.sr[2]) };

            let path_slice = unsafe { std::slice::from_raw_parts(path_ptr, path_len) };
            let path =
                std::str::from_utf8(path_slice).expect("Non UTF8 data provided for open syscall.");

            vm.reg.srr = sys::io::open(path, flags);
        }
        _ => panic!("Invalid syscall3: {syscall:?}"),
    }
}

pub fn syscall4(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        _ => panic!("Invalid syscall0: {syscall:?}"),
    }
}

pub fn syscall5(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        _ => panic!("Invalid syscall5: {syscall:?}"),
    }
}

pub fn syscall6(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        _ => panic!("Invalid syscall6: {syscall:?}"),
    }
}
