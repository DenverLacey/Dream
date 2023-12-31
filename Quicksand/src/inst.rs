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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgumentType {
    Lit32 = 0b00,
    Register = 0b01,
    Address = 0b10,
    Lit64 = 0b11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstructionSignature(u8);

impl Instruction {
    pub const fn sig1(arg1: ArgumentType) -> InstructionSignature {
        InstructionSignature(arg1 as u8)
    }

    pub const fn sig2(arg1: ArgumentType, arg2: ArgumentType) -> InstructionSignature {
        let arg1 = arg1 as u8;
        let arg2 = arg2 as u8;
        InstructionSignature((arg2 << 2) | arg1)
    }

    pub const fn sig3(
        arg1: ArgumentType,
        arg2: ArgumentType,
        arg3: ArgumentType,
    ) -> InstructionSignature {
        let arg1 = arg1 as u8;
        let arg2 = arg2 as u8;
        let arg3 = arg3 as u8;
        InstructionSignature((arg3 << 4) | (arg2 << 2) | arg1)
    }

    pub const fn sig4(
        arg1: ArgumentType,
        arg2: ArgumentType,
        arg3: ArgumentType,
        arg4: ArgumentType,
    ) -> InstructionSignature {
        let arg1 = arg1 as u8;
        let arg2 = arg2 as u8;
        let arg3 = arg3 as u8;
        let arg4 = arg4 as u8;
        InstructionSignature((arg4 << 6) | (arg3 << 4) | (arg2 << 2) | arg1)
    }
}

#[macro_export]
macro_rules! inst_sig {
    ($arg1:expr $(,)?) => {
        $crate::Instruction::sig1($arg1)
    };
    ($arg1:expr, $arg2:expr $(,)?) => {
        $crate::Instruction::sig2($arg1, $arg2)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::Instruction::sig3($arg1, $arg2, $arg3)
    };
    ($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr $(,)?) => {
        $crate::Instruction::sig4($arg1, $arg2, $arg3, $arg4)
    };
}

impl InstructionSignature {
    pub fn fst(self) -> ArgumentType {
        unsafe { std::mem::transmute(self.0 & 0x03) }
    }

    pub fn snd(self) -> ArgumentType {
        unsafe { std::mem::transmute((self.0 & 0x0C) >> 2) }
    }

    pub fn thd(self) -> ArgumentType {
        unsafe { std::mem::transmute((self.0 & 0x30) >> 4) }
    }

    pub fn frth(self) -> ArgumentType {
        unsafe { std::mem::transmute((self.0 & 0xC0) >> 6) }
    }

    pub fn get(self, index: usize) -> ArgumentType {
        assert!(index < 4);
        let shift = (index as u8) * 2;
        unsafe { std::mem::transmute((self.0 >> shift) & 0x03) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sig1() {
        let sig = Instruction::sig1(ArgumentType::Address);
        assert_eq!(sig, InstructionSignature(0b00000010));
    }

    #[test]
    fn sig2() {
        let sig = Instruction::sig2(ArgumentType::Address, ArgumentType::Address);
        assert_eq!(sig, InstructionSignature(0b00001010));
    }

    #[test]
    fn sig3() {
        let sig = Instruction::sig3(
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address,
        );
        assert_eq!(sig, InstructionSignature(0b00101010));
    }

    #[test]
    fn sig4() {
        let sig = Instruction::sig4(
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address,
        );
        assert_eq!(sig, InstructionSignature(0b10101010));
    }

    #[test]
    fn sig_macro() {
        let sig1 = inst_sig!(ArgumentType::Address);
        let sig2 = inst_sig!(ArgumentType::Address, ArgumentType::Address);
        let sig3 = inst_sig!(
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address
        );
        let sig4 = inst_sig!(
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address,
            ArgumentType::Address,
        );

        assert_eq!(sig1, InstructionSignature(0b00000010));
        assert_eq!(sig2, InstructionSignature(0b00001010));
        assert_eq!(sig3, InstructionSignature(0b00101010));
        assert_eq!(sig4, InstructionSignature(0b10101010));
    }

    #[test]
    fn fst() {
        let sig = inst_sig!(
            ArgumentType::Lit32,
            ArgumentType::Register,
            ArgumentType::Address,
            ArgumentType::Lit64
        );
        assert_eq!(sig.fst(), ArgumentType::Lit32);
    }

    #[test]
    fn snd() {
        let sig = inst_sig!(
            ArgumentType::Lit32,
            ArgumentType::Register,
            ArgumentType::Address,
            ArgumentType::Lit64
        );
        assert_eq!(sig.snd(), ArgumentType::Register);
    }

    #[test]
    fn thd() {
        let sig = inst_sig!(
            ArgumentType::Lit32,
            ArgumentType::Register,
            ArgumentType::Address,
            ArgumentType::Lit64
        );
        assert_eq!(sig.thd(), ArgumentType::Address);
    }

    #[test]
    fn frth() {
        let sig = inst_sig!(
            ArgumentType::Lit32,
            ArgumentType::Register,
            ArgumentType::Address,
            ArgumentType::Lit64
        );
        assert_eq!(sig.frth(), ArgumentType::Lit64);
    }

    #[test]
    fn get() {
        let sig = inst_sig!(
            ArgumentType::Lit32,
            ArgumentType::Register,
            ArgumentType::Address,
            ArgumentType::Lit64
        );
        assert_eq!(sig.get(0), ArgumentType::Lit32);
        assert_eq!(sig.get(1), ArgumentType::Register);
        assert_eq!(sig.get(2), ArgumentType::Address);
        assert_eq!(sig.get(3), ArgumentType::Lit64);
    }
}
