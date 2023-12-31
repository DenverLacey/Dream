use crate::errors::{Error, Result};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterType {
    X = 0x00, // Extra Special Purpose Registers e.g. the Z register.
    S = 0x20, // Syscall Registers.
    B = 0x40, // B-Registers. Index stored in lower 5 bits.
    W = 0x80, // W-Registers. Index stored in lower 5 bits.
    D = 0xC0, // D-Registers. Index stored in lower 5 bits.
    Q = 0xE0, // Q-Registers. Index stored in lower 5 bits.
}

pub const REGISTER_PREFIX_MASK: u8 = 0xE0;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyscallRegisterPrefix {
    RSI = 0x08, // RSI Register: The Syscall Index register used to specify syscall to execute.
    RSR = 0x10, // RSR Register: The Syscall Return register used to store return value of syscall.
    RSX = 0x00, // RSX-Registers: The registers used to pass arguments to syscalls.
}

pub const REGISTER_Z: u8 = RegisterType::X as u8 | 0x00;
pub const REGISTER_RSI: u8 = RegisterType::S as u8 | SyscallRegisterPrefix::RSI as u8;
pub const REGISTER_RSR: u8 = RegisterType::S as u8 | SyscallRegisterPrefix::RSR as u8;
pub const REGISTER_RSX: u8 = RegisterType::S as u8 | SyscallRegisterPrefix::RSX as u8;
pub const REGISTER_RS0: u8 = REGISTER_RSX | 0;
pub const REGISTER_RS1: u8 = REGISTER_RSX | 1;
pub const REGISTER_RS2: u8 = REGISTER_RSX | 2;
pub const REGISTER_RS3: u8 = REGISTER_RSX | 3;
pub const REGISTER_RS4: u8 = REGISTER_RSX | 4;
pub const REGISTER_RS5: u8 = REGISTER_RSX | 5;

pub const fn rsx(x: u8) -> Result<u8> {
    if x < 6 {
        Ok(REGISTER_RSX | x)
    } else {
        Err(Error::InvalidRegisterIndex)
    }
}

pub const fn gpr(reg_type: RegisterType, x: u8) -> Result<u8> {
    if x < 32 {
        Ok(reg_type as u8 | x)
    } else {
        Err(Error::InvalidRegisterIndex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sr0() {
        let result = rsx(0);
        assert!(matches!(result, Ok(REGISTER_SR0)));
    }

    #[test]
    fn sr1() {
        let result = rsx(1);
        assert!(matches!(result, Ok(REGISTER_SR1)));
    }

    #[test]
    fn sr2() {
        let result = rsx(2);
        assert!(matches!(result, Ok(REGISTER_SR2)));
    }

    #[test]
    fn sr3() {
        let result = rsx(3);
        assert!(matches!(result, Ok(REGISTER_SR3)));
    }

    #[test]
    fn sr4() {
        let result = rsx(4);
        assert!(matches!(result, Ok(REGISTER_SR4)));
    }

    #[test]
    fn sr5() {
        let result = rsx(5);
        assert!(matches!(result, Ok(REGISTER_SR5)));
    }

    #[test]
    fn sr6() {
        let result = rsx(6);
        assert!(matches!(result, Err(Error::InvalidRegisterIndex)));
    }

    #[test]
    fn br0() {
        let result = gpr(RegisterType::B, 0);
        assert!(matches!(result, Ok(0x40)));
    }

    #[test]
    fn br31() {
        let result = gpr(RegisterType::B, 31);
        assert!(matches!(result, Ok(0x5F)));
    }
}
