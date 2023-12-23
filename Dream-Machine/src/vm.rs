const STACK_SIZE: usize = 4 * 1024;

#[derive(Debug, Default)]
pub struct VM {
    pub reg: Registers,
    pub stack: Stack<STACK_SIZE>,
}

#[derive(Debug, Default)]
pub struct Registers {
    pub z: u8,
    pub sri: u16,
    pub srr: u64,
    pub sr: [u64; 6],
    pub gen: General,
}

const NUM_8_BYTE_REGISTERS: usize = 32;

pub union General {
    b: [u8; NUM_8_BYTE_REGISTERS * 8],
    w: [u16; NUM_8_BYTE_REGISTERS * 4],
    d: [u32; NUM_8_BYTE_REGISTERS * 2],
    q: [u64; NUM_8_BYTE_REGISTERS * 1],
}

impl std::default::Default for General {
    fn default() -> Self {
        Self {
            q: [0; NUM_8_BYTE_REGISTERS],
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
