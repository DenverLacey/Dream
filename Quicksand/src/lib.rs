#[repr(u8)]
pub enum Instruction {
    NOP = 0x00, // No-Op: Does nothing.

    LDB = 0x01, // Load B-Register: Loads 1 byte into a B-Register.
    LDW = 0x02, // Load W-Register: Loads 2 bytes into a W-Register.
    LDD = 0x03, // Load D-Register: Loads 4 bytes into a D-Register.
    LDQ = 0x04, // Load Q-Register: Loads 8 bytes into a Q-Register.

    PUSH = 0x10, // Push: Push a value onto the stack.
    POP = 0x11,  // Pop: Pop a value from the stack and copy into a register.

    MAX = 0x7F, // This is the maximum value for an instruction. The top-most bit is reserved.
}

#[repr(u8)]
pub enum RegisterPrefix {
    X = 0x00, // Extra Special Purpose Registers e.g. the Z register.
    S = 0x20, // Syscall Registers.
    B = 0x40, // B-Registers. Requires second byte for index.
    W = 0x80, // W-Registers. Requires second byte for index.
    D = 0xC0, // D-Registers. Index stored in lower 5 bits.
    Q = 0xE0, // Q-Registers. Index stored in lower 5 bits.
}

pub const REGISTER_PREFIX_MASK: u8 = 0xE0;

#[repr(u8)]
pub enum SyscallRegisterPrefix {
    SRI = 0x08, // SRI Register: The Syscall Index register used to specify syscall to execute.
    SRR = 0x10, // SRR Register: The Syscall Return register used to store return value of syscall.
    SRX = 0x00, // SR-Registers: The registers used to pass arguments to syscalls.
}

pub const REGISTER_Z: u8 = RegisterPrefix::X as u8 | 0x00;
pub const REGISTER_SRI: u8 = RegisterPrefix::S as u8 | SyscallRegisterPrefix::SRI as u8;
pub const REGISTER_SRR: u8 = RegisterPrefix::S as u8 | SyscallRegisterPrefix::SRR as u8;
pub const REGISTER_SRX: u8 = RegisterPrefix::S as u8 | SyscallRegisterPrefix::SRX as u8;
pub const REGISTER_SR0: u8 = REGISTER_SRX | 0;
pub const REGISTER_SR1: u8 = REGISTER_SRX | 1;
pub const REGISTER_SR2: u8 = REGISTER_SRX | 2;
pub const REGISTER_SR3: u8 = REGISTER_SRX | 3;
pub const REGISTER_SR4: u8 = REGISTER_SRX | 4;
pub const REGISTER_SR5: u8 = REGISTER_SRX | 5;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
