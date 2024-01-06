#[repr(u8)]
pub enum Instruction {
    NoOp = 0x00,     // Does nothing.
    Move = 0x01,     // Move a value into a register.
    MoveImm = 0x02,  // Move an immediate value into a register.
    MoveAddr = 0x03, // Move a value into a register via an address.
    Clear = 0x04,    // Set a register to zero.
    Set = 0x05,      // Set a register to one.
    Push = 0x06,     // Push a value onto the stack.
    PushImm = 0x07,  // Push an immediate value onto the stack.
    Pop = 0x08,      // Pop a value from the stack and copy into a register.
    Syscall0 = 0x10, // Perform syscall with 0 arguments.
    Syscall1 = 0x11, // Perform syscall with 1 argument.
    Syscall2 = 0x12, // Perform syscall with 2 arguments.
    Syscall3 = 0x13, // Perform syscall with 3 arguments.
    Syscall4 = 0x14, // Perform syscall with 4 arguments.
    Syscall5 = 0x15, // Perform syscall with 5 arguments.
    Syscall6 = 0x16, // Perform syscall with 6 arguments.
    Ret = 0x20,      // Returns from the current procedure.
    MAX = 0x7F,      // This is the maximum value for an instruction. The top-most bit is reserved.
}

pub const INST_ALT_MODE: u8 = 0x80;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperandType {
    // Lit32 = 0b00,  NOTE: Removing this to allow for zero to mean None.
    Register = 0b01,
    Address = 0b10,
    Lit64 = 0b11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstructionSignature(u8);

impl InstructionSignature {
    pub const fn to_u8(self) -> u8 {
        self.0
    }
}

impl Instruction {
    pub const fn sig1(op1: OperandType) -> InstructionSignature {
        InstructionSignature(op1 as u8)
    }

    pub const fn sig2(op1: OperandType, op2: OperandType) -> InstructionSignature {
        let op1 = op1 as u8;
        let op2 = op2 as u8;
        InstructionSignature((op2 << 2) | op1)
    }

    pub const fn sig3(
        op1: OperandType,
        op2: OperandType,
        op3: OperandType,
    ) -> InstructionSignature {
        let op1 = op1 as u8;
        let op2 = op2 as u8;
        let op3 = op3 as u8;
        InstructionSignature((op3 << 4) | (op2 << 2) | op1)
    }

    pub const fn sig4(
        op1: OperandType,
        op2: OperandType,
        op3: OperandType,
        op4: OperandType,
    ) -> InstructionSignature {
        let op1 = op1 as u8;
        let op2 = op2 as u8;
        let op3 = op3 as u8;
        let op4 = op4 as u8;
        InstructionSignature((op4 << 6) | (op3 << 4) | (op2 << 2) | op1)
    }
}

#[macro_export]
macro_rules! inst_sig {
    ($op1:expr $(,)?) => {
        $crate::Instruction::sig1($op1)
    };
    ($op1:expr, $op2:expr $(,)?) => {
        $crate::Instruction::sig2($op1, $op2)
    };
    ($op1:expr, $op2:expr, $op3:expr $(,)?) => {
        $crate::Instruction::sig3($op1, $op2, $op3)
    };
    ($op1:expr, $op2:expr, $op3:expr, $op4:expr $(,)?) => {
        $crate::Instruction::sig4($op1, $op2, $op3, $op4)
    };
}

impl InstructionSignature {
    pub fn fst(self) -> OperandType {
        unsafe { std::mem::transmute(self.0 & 0x03) }
    }

    pub fn snd(self) -> OperandType {
        unsafe { std::mem::transmute((self.0 & 0x0C) >> 2) }
    }

    pub fn thd(self) -> OperandType {
        unsafe { std::mem::transmute((self.0 & 0x30) >> 4) }
    }

    pub fn frth(self) -> OperandType {
        unsafe { std::mem::transmute((self.0 & 0xC0) >> 6) }
    }

    pub fn get(self, index: usize) -> OperandType {
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
        let sig = Instruction::sig1(OperandType::Address);
        assert_eq!(sig, InstructionSignature(0b00000010));
    }

    #[test]
    fn sig2() {
        let sig = Instruction::sig2(OperandType::Address, OperandType::Address);
        assert_eq!(sig, InstructionSignature(0b00001010));
    }

    #[test]
    fn sig3() {
        let sig = Instruction::sig3(
            OperandType::Address,
            OperandType::Address,
            OperandType::Address,
        );
        assert_eq!(sig, InstructionSignature(0b00101010));
    }

    #[test]
    fn sig4() {
        let sig = Instruction::sig4(
            OperandType::Address,
            OperandType::Address,
            OperandType::Address,
            OperandType::Address,
        );
        assert_eq!(sig, InstructionSignature(0b10101010));
    }

    #[test]
    fn sig_macro() {
        let sig1 = inst_sig!(OperandType::Address);
        let sig2 = inst_sig!(OperandType::Address, OperandType::Address);
        let sig3 = inst_sig!(
            OperandType::Address,
            OperandType::Address,
            OperandType::Address
        );
        let sig4 = inst_sig!(
            OperandType::Address,
            OperandType::Address,
            OperandType::Address,
            OperandType::Address,
        );

        assert_eq!(sig1, InstructionSignature(0b00000010));
        assert_eq!(sig2, InstructionSignature(0b00001010));
        assert_eq!(sig3, InstructionSignature(0b00101010));
        assert_eq!(sig4, InstructionSignature(0b10101010));
    }

    #[test]
    fn fst() {
        let sig = inst_sig!(
            OperandType::Lit64,
            OperandType::Register,
            OperandType::Address,
            OperandType::Lit64
        );
        assert_eq!(sig.fst(), OperandType::Lit64);
    }

    #[test]
    fn snd() {
        let sig = inst_sig!(
            OperandType::Lit64,
            OperandType::Register,
            OperandType::Address,
            OperandType::Lit64
        );
        assert_eq!(sig.snd(), OperandType::Register);
    }

    #[test]
    fn thd() {
        let sig = inst_sig!(
            OperandType::Lit64,
            OperandType::Register,
            OperandType::Address,
            OperandType::Lit64
        );
        assert_eq!(sig.thd(), OperandType::Address);
    }

    #[test]
    fn frth() {
        let sig = inst_sig!(
            OperandType::Lit64,
            OperandType::Register,
            OperandType::Address,
            OperandType::Lit64
        );
        assert_eq!(sig.frth(), OperandType::Lit64);
    }

    #[test]
    fn get() {
        let sig = inst_sig!(
            OperandType::Lit64,
            OperandType::Register,
            OperandType::Address,
            OperandType::Lit64
        );
        assert_eq!(sig.get(0), OperandType::Lit64);
        assert_eq!(sig.get(1), OperandType::Register);
        assert_eq!(sig.get(2), OperandType::Address);
        assert_eq!(sig.get(3), OperandType::Lit64);
    }
}
