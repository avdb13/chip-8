fn main() {
    println!("Hello, world!");
}

static RAM: [u8; 4096] = [0; 4096];
static DISPLAY: [[bool; 32]; 64] = [[false; 32]; 64];

// points at the current instruction in memory
static PC: u16 = 0u16;

// used to call subroutines/functions and return from them
static REGISTER: u16 = 0u16;

// functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
static TIMER: u8 = 0u8;

// VF is also used as a flag register
// many instructions will set it to either 1 or 0 based on some rule,
// for example using it as a carry flag
static VARIABLE_REGISTER: [u8; 16] = [0u8; 16];

static FONT: [[u32; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F

                                    //  [0xE0, 0xA0, 0xA0, 0xA0, 0xE0],
                                    //  [0x40, 0x40, 0x40, 0x40, 0x40],
                                    //  [0xE0, 0x20, 0xA0, 0x80, 0xE0],
                                    //  [0xE0, 0x20, 0xE0, 0x20, 0xE0],
                                    //  [0x00, 0x00, 0x00, 0x00, 0x00],
];

pub struct Sprite([u8; 15]);

// used to call subroutines/functions and return from them
pub struct Stack {
    inner: [Option<u16>; 64],
    index: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            inner: [None; 64],
            index: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.inner.len() <= self.index {
            panic!("buffer overflow");
        }

        self.inner[self.index] = Some(value);
        self.index += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.index == 0 {
            panic!("buffer underflow");
        }

        self.index -= 1;
        self.inner[self.index].take().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        let mut contents: [_; 64] = core::array::from_fn(|i| i as u16 + 1);
        let mut result = [0u16; 64];

        for i in contents {
            stack.push(i);
        }

        for i in 0..contents.len() {
            result[i] = stack.pop();
        }
        contents.reverse();

        assert_eq!(result, contents);
    }

    #[test]
    #[should_panic]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        let contents: [_; 65] = core::array::from_fn(|i| i as u16 + 1);

        for i in contents {
            stack.push(i);
        }
    }

    #[test]
    #[should_panic]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        let _ = stack.pop();
    }
}
