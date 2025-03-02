mod helpers;
use helpers::stack::Stack;


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
        Self {
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
    }
}