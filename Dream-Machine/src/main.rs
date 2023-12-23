mod syscalls;
mod vm;

use syscalls::*;
use vm::*;

fn main() {
    let mut dvm = VM::default();
    dvm.reg.sri = Syscall::Write as u16;
    syscall3(&mut dvm);
}
