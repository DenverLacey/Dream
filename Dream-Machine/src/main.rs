mod sys;
mod syscalls;
mod vm;

use sys::{FileID, OpenFlags, STDOUT};
use syscalls::*;
use vm::*;

fn main() {
    let mut dvm = VM::default();
    let dvm = &mut dvm;

    {
        dvm.reg.sri = Syscall::Write as u16;
        dvm.reg.sr[0] = STDOUT;

        let msg = "Hello from the dream machine\n";
        let bytes = msg.as_bytes();

        dvm.reg.sr[1] = bytes.as_ptr() as u64;
        dvm.reg.sr[2] = bytes.len() as u64;

        syscall3(dvm);
    }

    {
        let path = "test.txt";
        let path_bytes = path.as_bytes();

        dvm.reg.sri = Syscall::Open as u16;
        dvm.reg.sr[0] = path_bytes.as_ptr() as u64;
        dvm.reg.sr[1] = path_bytes.len() as u64;
        dvm.reg.sr[2] = (OpenFlags::Create | OpenFlags::Write) as u64;
        syscall3(dvm);

        let fid: FileID = dvm.reg.srr;

        let msg = "Hello, test.txt!\nThis was done using Dream Machine syscalls.\n";
        let msg_bytes = msg.as_bytes();

        dvm.reg.sri = Syscall::Write as u16;
        dvm.reg.sr[0] = fid;
        dvm.reg.sr[1] = msg_bytes.as_ptr() as u64;
        dvm.reg.sr[2] = msg_bytes.len() as u64;
        syscall3(dvm);

        dvm.reg.sri = Syscall::Close as u16;
        syscall1(dvm);
    }

    {
        let path = "test.txt";
        let path_bytes = path.as_bytes();

        dvm.reg.sri = Syscall::Open as u16;
        dvm.reg.sr[0] = path_bytes.as_ptr() as u64;
        dvm.reg.sr[1] = path_bytes.len() as u64;
        dvm.reg.sr[2] = OpenFlags::Read as u64;
        syscall3(dvm);

        let fid: FileID = dvm.reg.srr;

        let buf = &mut [0u8; 80];

        dvm.reg.sri = Syscall::Read as u16;
        dvm.reg.sr[0] = fid;
        dvm.reg.sr[1] = buf.as_mut_ptr() as u64;
        dvm.reg.sr[2] = buf.len() as u64;
        syscall3(dvm);

        let len = dvm.reg.srr;

        dvm.reg.sri = Syscall::Write as u16;
        dvm.reg.sr[0] = STDOUT;
        dvm.reg.sr[1] = buf.as_ptr() as u64;
        dvm.reg.sr[2] = len;
        syscall3(dvm);

        dvm.reg.sri = Syscall::Close as u16;
        dvm.reg.sr[0] = fid;
        syscall1(dvm);
    }
}
