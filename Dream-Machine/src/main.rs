mod sys;
mod syscalls;
mod vm;

use syscalls::*;
use vm::*;

fn main() {
    let mut dvm = VM::default();
    dvm.reg.sri = Syscall::Write as u16;
    dvm.reg.sr[0] = 1;

    let msg = "Hello from the dream machine\n";
    let bytes = msg.as_bytes();

    dvm.reg.sr[1] = bytes.as_ptr() as u64;
    dvm.reg.sr[2] = bytes.len() as u64;

    syscall3(&mut dvm);
}
