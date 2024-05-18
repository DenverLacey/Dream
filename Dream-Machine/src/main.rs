use std::{fs::File, io::{BufRead, BufReader, Read}};

use clap::Parser;

mod sys;
mod syscalls;
mod vm;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Dream file to execute
    file: String,

    /// Print human-readable disassembly of the dream file
    #[arg(long = "emit-disassembly")]
    emit_disassembly: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(dasm_path) = cli.emit_disassembly {
        let rdr = BufReader::new(File::open(&cli.file).unwrap());
        let mut dasm_file = File::create(dasm_path).unwrap();
        morpheus::disassemble(rdr.bytes().map(Result::unwrap), &mut dasm_file).unwrap();
    }

    println!("The file you want to run is {:?}.", cli.file);
}

#[cfg(test)]
mod tests {
    use crate::sys::{FileID, OpenFlags, STDOUT};
    use crate::syscalls::*;
    use crate::vm::*;

    #[test]
    fn file_operations() {
        let mut dvm = VM::default();
        let dvm = &mut dvm;

        {
            dvm.reg.rsi = Syscall::Write as u16;
            dvm.reg.rs[0] = STDOUT;

            let msg = "Hello from the dream machine\n";
            let bytes = msg.as_bytes();

            dvm.reg.rs[1] = bytes.as_ptr() as u64;
            dvm.reg.rs[2] = bytes.len() as u64;

            syscall3(dvm);
        }

        {
            let path = "tests/test.txt";
            let path_bytes = path.as_bytes();

            dvm.reg.rsi = Syscall::Open as u16;
            dvm.reg.rs[0] = path_bytes.as_ptr() as u64;
            dvm.reg.rs[1] = path_bytes.len() as u64;
            dvm.reg.rs[2] = (OpenFlags::Create | OpenFlags::Write) as u64;
            syscall3(dvm);

            let fid: FileID = dvm.reg.rsr;

            let msg = "Hello, test.txt!\nThis was done using Dream Machine syscalls.\n";
            let msg_bytes = msg.as_bytes();

            dvm.reg.rsi = Syscall::Write as u16;
            dvm.reg.rs[0] = fid;
            dvm.reg.rs[1] = msg_bytes.as_ptr() as u64;
            dvm.reg.rs[2] = msg_bytes.len() as u64;
            syscall3(dvm);

            dvm.reg.rsi = Syscall::Close as u16;
            syscall1(dvm);
        }

        {
            let path = "tests/test.txt";
            let path_bytes = path.as_bytes();

            dvm.reg.rsi = Syscall::Open as u16;
            dvm.reg.rs[0] = path_bytes.as_ptr() as u64;
            dvm.reg.rs[1] = path_bytes.len() as u64;
            dvm.reg.rs[2] = OpenFlags::Read as u64;
            syscall3(dvm);

            let fid: FileID = dvm.reg.rsr;

            let buf = &mut [0u8; 80];

            dvm.reg.rsi = Syscall::Read as u16;
            dvm.reg.rs[0] = fid;
            dvm.reg.rs[1] = buf.as_mut_ptr() as u64;
            dvm.reg.rs[2] = buf.len() as u64;
            syscall3(dvm);

            let len = dvm.reg.rsr;

            dvm.reg.rsi = Syscall::Write as u16;
            dvm.reg.rs[0] = STDOUT;
            dvm.reg.rs[1] = buf.as_ptr() as u64;
            dvm.reg.rs[2] = len;
            syscall3(dvm);

            dvm.reg.rsi = Syscall::Close as u16;
            dvm.reg.rs[0] = fid;
            syscall1(dvm);
        }
    }
}

