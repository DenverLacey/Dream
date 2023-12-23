// https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

use crate::sys;
use crate::vm::VM;

#[repr(u16)]
#[derive(Debug)]
pub enum Syscall {
    Read = 0,  // fd, buf, size
    Write = 1, // fd, buf, size
    Open = 2,  // filename, flags, mode
    Close = 3, // fd
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
        Syscall::Close => todo!(),
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
        Syscall::Read => todo!(),
        Syscall::Write => {
            let fid = vm.reg.sr[0] as sys::FileID;
            let bytes_to_write = unsafe {
                std::slice::from_raw_parts(vm.reg.sr[1] as *const u8, vm.reg.sr[2] as usize)
            };
            sys::io::write(fid, bytes_to_write);
        }
        Syscall::Open => todo!(),
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
