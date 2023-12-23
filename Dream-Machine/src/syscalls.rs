// https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

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
    todo!()
}

pub fn syscall1(vm: &mut VM) {
    todo!()
}

pub fn syscall2(vm: &mut VM) {
    todo!()
}

pub fn syscall3(vm: &mut VM) {
    let syscall: Syscall = unsafe { std::mem::transmute(vm.reg.sri) };
    match syscall {
        Syscall::Read => todo!(),
        Syscall::Write => todo!(),
        Syscall::Open => todo!(),
        _ => panic!("Invalid syscall3: {syscall:?}"),
    }
}

pub fn syscall4(vm: &mut VM) {
    todo!()
}

pub fn syscall5(vm: &mut VM) {
    todo!()
}

pub fn syscall6(vm: &mut VM) {
    todo!()
}
