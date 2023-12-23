mod sys;
mod syscalls;
mod vm;

use sys::{FileFlags, FileID};
use syscalls::*;
use vm::*;

fn main() {
    let mut dvm = VM::default();

    {
        dvm.reg.sri = Syscall::Write as u16;
        dvm.reg.sr[0] = 1;

        let msg = "Hello from the dream machine\n";
        let bytes = msg.as_bytes();

        dvm.reg.sr[1] = bytes.as_ptr() as u64;
        dvm.reg.sr[2] = bytes.len() as u64;

        syscall3(&mut dvm);
    }

    {
        let path = "test.txt";
        let path_bytes = path.as_bytes();

        dvm.reg.sri = Syscall::Open as u16;
        dvm.reg.sr[0] = path_bytes.as_ptr() as u64;
        dvm.reg.sr[1] = path_bytes.len() as u64;
        dvm.reg.sr[2] = (FileFlags::Create | FileFlags::Write) as u64;
        syscall3(&mut dvm);

        let fid: FileID = dvm.reg.srr;

        let msg = "Hello, test.txt!\nThis was done using Dream Machine syscalls.\n";
        let msg_bytes = msg.as_bytes();

        dvm.reg.sri = Syscall::Write as u16;
        dvm.reg.sr[0] = fid;
        dvm.reg.sr[1] = msg_bytes.as_ptr() as u64;
        dvm.reg.sr[2] = msg_bytes.len() as u64;
        syscall3(&mut dvm);

        dvm.reg.sri = Syscall::Close as u16;
        syscall1(&mut dvm);
    }
}
