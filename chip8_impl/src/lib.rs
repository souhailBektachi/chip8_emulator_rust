mod helpers;
use helpers::stack::Stack;

const FONT_SIZE : usize = 80;

const FONTSET: [u8; FONT_SIZE] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;


const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;



pub struct Emulator{
    pc : u16,
    memory : [u8; MEMORY_SIZE],
    screen : [bool ; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    v_register  : [u8; NUM_REGISTERS],
    i_register : u16,
    stack: Stack,
    
    keypad : [bool; NUM_KEYS],
    delay_timer : u8,
    sound_timer : u8,

}


const START_ADDRESS: u16 = 0x200;

impl Emulator {


    pub fn new() -> Self {
        let mut emu = Self{
            pc : START_ADDRESS,
            memory : [0; MEMORY_SIZE],
            screen : [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_register : [0; NUM_REGISTERS],
            i_register : 0,
            stack : Stack::new(),
            keypad : [false; NUM_KEYS],
            delay_timer : 0,
            sound_timer : 0,
        }
        emu.memory[..FONT_SIZE].copy_from_slice(&FONTSET);

        emu 
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS;
        self.memory = [0; MEMORY_SIZE];
        self.screen = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.v_register = [0; NUM_REGISTERS];
        self.i_register = 0;
        self.stack = Stack::new();
        self.keypad = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.memory[..FONT_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        todo!();
    }

    fn fetch_opcode(&mut self)->u16 {
        let higher_byte = self.memory[self.pc as usize] as u16;
        let lower_byte = self.memory[self.pc as usize + 1] as u16;
        let opcode = (higher_byte << 8) | lower_byte; 
        self.pc += 2;
        opcode

    }
}