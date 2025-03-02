

pub struct Stack {
    stack : [u16; 16],
    stack_pointer : usize,
}

impl Stack {

    pub fn new() -> Self {
        Self {
            stack : [0; 16],
            stack_pointer : 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

    
}