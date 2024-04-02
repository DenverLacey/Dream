use quicksand::{Register, RegisterType};

#[derive(Clone, Copy, Default, Debug)]
pub struct RegisterAllocator {
    next_b: u8,
    next_w: u8,
    next_d: u8,
    next_q: u8,
}

pub struct RegisterArena<'ator> {
    allocator: &'ator mut RegisterAllocator,
    reset_b: u8,
    reset_w: u8,
    reset_d: u8,
    reset_q: u8,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            next_b: 0,
            next_w: 0,
            next_d: 0,
            next_q: 0,
        }
    }

    pub fn start_arena(&mut self) -> RegisterArena<'_> {
        RegisterArena {
            reset_b: self.next_b,
            reset_w: self.next_w,
            reset_d: self.next_d,
            reset_q: self.next_q,
            allocator: self,
        }
    }
}

macro_rules! inc_with_max {
    ($value:expr, $max:expr) => {{
        let x = $value;
        if x >= $max {
            panic!("Too many allocators in use.");
        }
        $value += 1;
        x
    }};
}

impl<'ator> RegisterArena<'ator> {
    pub fn new_arena(&mut self) -> RegisterArena<'_> {
        RegisterArena {
            allocator: &mut self.allocator,
            reset_b: self.reset_b,
            reset_w: self.reset_w,
            reset_d: self.reset_d,
            reset_q: self.reset_q,
        }
    }

    pub fn next(&mut self, r#type: RegisterType) -> Register {
        let idx = match r#type {
            RegisterType::X => panic!("Cannot allocate an X register."),
            RegisterType::S => panic!("Cannot allocate an S register."),
            RegisterType::B => inc_with_max!(self.allocator.next_b, Register::MAX),
            RegisterType::W => inc_with_max!(self.allocator.next_w, Register::MAX),
            RegisterType::D => inc_with_max!(self.allocator.next_d, Register::MAX),
            RegisterType::Q => inc_with_max!(self.allocator.next_q, Register::MAX),
        };

        Register::new(r#type, idx).expect("INTERNAL ERROR: failed to bounds check register before calling Register::new")
    }
}

impl<'ator> Drop for RegisterArena<'ator> {
    fn drop(&mut self) {
        self.allocator.next_b = self.reset_b;
        self.allocator.next_w = self.reset_w;
        self.allocator.next_d = self.reset_d;
        self.allocator.next_q = self.reset_q;
    }
}

