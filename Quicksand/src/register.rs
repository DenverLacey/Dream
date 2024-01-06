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

pub const SYSCALL_REGISTER_PREFIX_MASK: u8 = 0x18;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register(u8);

impl Register {
    const RSX: u8 = RegisterType::S as u8 | SyscallRegisterPrefix::RSX as u8;

    pub const Z: Register = Register(RegisterType::X as u8 | 0x00);
    pub const RSI: Register = Register(RegisterType::S as u8 | SyscallRegisterPrefix::RSI as u8);
    pub const RSR: Register = Register(RegisterType::S as u8 | SyscallRegisterPrefix::RSR as u8);
    pub const RS0: Register = Register(Self::RSX | 0);
    pub const RS1: Register = Register(Self::RSX | 1);
    pub const RS2: Register = Register(Self::RSX | 2);
    pub const RS3: Register = Register(Self::RSX | 3);
    pub const RS4: Register = Register(Self::RSX | 4);
    pub const RS5: Register = Register(Self::RSX | 5);

    pub const fn new(reg_type: RegisterType, x: u8) -> Result<Self> {
        match reg_type {
            RegisterType::X => Ok(Register(reg_type as u8 | x)),
            RegisterType::S if x < 6 => Ok(Register(reg_type as u8 | Self::RSX | x)),
            RegisterType::B | RegisterType::W | RegisterType::D | RegisterType::Q if x < 32 => {
                Ok(Register(reg_type as u8 | x))
            }
            _ => Err(Error::InvalidRegisterIndex),
        }
    }

    pub const fn to_u8(self) -> u8 {
        self.0
    }

    pub const fn to_u64(self) -> u64 {
        self.0 as u64
    }

    pub const fn is_x(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::X as u8
    }

    pub const fn is_s(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::S as u8
    }

    pub const fn is_rsx(self) -> bool {
        const MASK: u8 = REGISTER_PREFIX_MASK | SYSCALL_REGISTER_PREFIX_MASK;
        (self.0 & MASK) == (RegisterType::S as u8 | SyscallRegisterPrefix::RSX as u8)
    }

    pub const fn is_b(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::B as u8
    }

    pub const fn is_w(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::W as u8
    }

    pub const fn is_d(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::D as u8
    }

    pub const fn is_q(self) -> bool {
        self.0 & REGISTER_PREFIX_MASK as u8 == RegisterType::Q as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sr0() {
        let result = Register::new(RegisterType::S, 0);
        assert!(matches!(result, Ok(Register::RS0)));
    }

    #[test]
    fn sr1() {
        let result = Register::new(RegisterType::S, 1);
        assert!(matches!(result, Ok(Register::RS1)));
    }

    #[test]
    fn sr2() {
        let result = Register::new(RegisterType::S, 2);
        assert!(matches!(result, Ok(Register::RS2)));
    }

    #[test]
    fn sr3() {
        let result = Register::new(RegisterType::S, 3);
        assert!(matches!(result, Ok(Register::RS3)));
    }

    #[test]
    fn sr4() {
        let result = Register::new(RegisterType::S, 4);
        assert!(matches!(result, Ok(Register::RS4)));
    }

    #[test]
    fn sr5() {
        let result = Register::new(RegisterType::S, 5);
        assert!(matches!(result, Ok(Register::RS5)));
    }

    #[test]
    fn sr6() {
        let result = Register::new(RegisterType::S, 6);
        assert!(matches!(result, Err(Error::InvalidRegisterIndex)));
    }

    #[test]
    fn br0() {
        let result = Register::new(RegisterType::B, 0);
        assert!(matches!(result, Ok(Register(0x40))));
    }

    #[test]
    fn br31() {
        let result = Register::new(RegisterType::B, 31);
        assert!(matches!(result, Ok(Register(0x5F))));
    }

    #[test]
    pub fn is_x() {
        let x = Register::new(RegisterType::X, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        assert!(x.is_x());
        assert!(!b.is_x());
    }

    #[test]
    pub fn is_s() {
        let s = Register::new(RegisterType::S, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        assert!(s.is_s());
        assert!(!b.is_s());
    }

    #[test]
    pub fn is_rsx() {
        let s = Register::new(RegisterType::S, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        let rsi = Register::RSI;
        assert!(s.is_rsx());
        assert!(!b.is_rsx());
        assert!(!rsi.is_rsx());
    }

    #[test]
    pub fn is_b() {
        let b = Register::new(RegisterType::B, 0).unwrap();
        let w = Register::new(RegisterType::W, 0).unwrap();
        assert!(b.is_b());
        assert!(!w.is_b());
    }

    #[test]
    pub fn is_w() {
        let w = Register::new(RegisterType::W, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        assert!(w.is_w());
        assert!(!b.is_w());
    }

    #[test]
    pub fn is_d() {
        let d = Register::new(RegisterType::D, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        assert!(d.is_d());
        assert!(!b.is_d());
    }

    #[test]
    pub fn is_q() {
        let q = Register::new(RegisterType::Q, 0).unwrap();
        let b = Register::new(RegisterType::B, 0).unwrap();
        assert!(q.is_q());
        assert!(!b.is_q());
    }
}
