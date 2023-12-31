const STACK_SIZE: usize = 4 * 1024;
const NUM_RSX_REGISTERS: usize = 6;
const NUM_REGISTERS_PER_SIZE: usize = 32;

#[derive(Debug, Default)]
pub struct VM {
    pub reg: Registers,
    pub stack: Stack<STACK_SIZE>,
}

#[derive(Debug, Default)]
pub struct Registers {
    pub z: u8,
    pub rsi: u16,
    pub rsr: u64,
    pub rs: [u64; NUM_RSX_REGISTERS],
    pub r: General,
}

#[repr(packed)]
pub struct General {
    b: [u8; NUM_REGISTERS_PER_SIZE],
    w: [u16; NUM_REGISTERS_PER_SIZE],
    d: [u32; NUM_REGISTERS_PER_SIZE],
    q: [u64; NUM_REGISTERS_PER_SIZE],
}

impl std::default::Default for General {
    fn default() -> Self {
        Self {
            b: [0; NUM_REGISTERS_PER_SIZE],
            w: [0; NUM_REGISTERS_PER_SIZE],
            d: [0; NUM_REGISTERS_PER_SIZE],
            q: [0; NUM_REGISTERS_PER_SIZE],
        }
    }
}

impl std::fmt::Debug for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[...]")
    }
}

#[derive(Debug)]
pub struct Stack<const N: usize> {
    allocated: usize,
    bytes: [u8; N],
}

impl<const N: usize> std::default::Default for Stack<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Stack<N> {
    pub fn new() -> Self {
        Self {
            allocated: 0,
            bytes: [0; N],
        }
    }

    pub fn push<T: Copy>(&mut self, value: T) -> Result<(), VMError> {
        let ptr = &value as *const T as *const () as *const u8;
        let bytes = unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of::<T>()) };
        self.push_bytes(bytes)?;
        Ok(())
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), VMError> {
        let begin = self.allocated;
        let end = begin + bytes.len();

        if end >= N {
            return Err(VMError::StackOverflow);
        }

        self.bytes[begin..end].copy_from_slice(bytes);
        self.allocated += bytes.len();
        Ok(())
    }

    pub fn pop<T: Copy>(&mut self) -> Result<T, VMError> {
        let bytes = self.pop_bytes(std::mem::size_of::<T>())?;
        let ptr = bytes.as_ptr() as *const T;
        Ok(unsafe { *ptr })
    }

    pub fn pop_bytes(&mut self, n: usize) -> Result<&[u8], VMError> {
        if n >= self.allocated {
            return Err(VMError::StackUnderflow);
        }

        let end = self.allocated;
        let begin = end - n;

        let bytes = &self.bytes[begin..end];
        self.allocated = begin;

        Ok(bytes)
    }
}

pub enum VMError {
    StackOverflow,
    StackUnderflow,
}
