mod helpers;
use core::num;

use helpers::stack::Stack;
use rand::random;
const FONT_SIZE: usize = 80;

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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const NUM_KEYS: usize = 16;

pub struct Emulator {
    pc: u16,
    memory: [u8; MEMORY_SIZE],
    screen: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    v_register: [u8; NUM_REGISTERS],
    i_register: u16,
    stack: Stack,

    keypad: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

const START_ADDRESS: u16 = 0x200;

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            pc: START_ADDRESS,
            memory: [0; MEMORY_SIZE],
            screen: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_register: [0; NUM_REGISTERS],
            i_register: 0,
            stack: Stack::new(),
            keypad: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
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
        self.execute(opcode);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.memory[self.pc as usize] as u16;
        let lower_byte = self.memory[self.pc as usize + 1] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }
    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2: u16 = (op & 0x0F00) >> 8;
        let d3: u16 = (op & 0x00F0) >> 4;
        let d4: u16 = op & 0x000F;

        match (d1, d2, d3, d4) {
            (0, 0, 0, 0) => return,
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }
            (2, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            (3, _, _, _) => {
                let x: usize = d2 as usize;
                let nn: u8 = (op & 0x00FF) as u8;
                if self.v_register[x] == nn {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => {
                let x: usize = d2 as usize;
                let nn = (op & 0x00FF) as u8;
                if self.v_register[x] != nn {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => {
                let x: usize = d2 as usize;
                let y: usize = d3 as usize;
                if self.v_register[x] == self.v_register[y] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                let x: usize = d2 as usize;
                let nn: u8 = (op & 0x00FF) as u8;
                self.v_register[x] = nn;
            }
            (7, _, _, _) => {
                let x: usize = d2 as usize;
                let nn: u8 = (op & 0x00FF) as u8;
                self.v_register[x] = self.v_register[x].wrapping_add(nn);
            }
            (8, _, _, 0) => {
                let x: usize = d2 as usize;
                let y: usize = d3 as usize;
                self.v_register[x] = self.v_register[y];
            }
            (8, _, _, 1) => {
                let x: usize = d2 as usize;
                let y: usize = d3 as usize;
                self.v_register[x] |= self.v_register[y];
            }
            (8, _, _, 2) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_register[x] &= self.v_register[y];
            }
            (8, _, _, 3) => {
                let x = d2 as usize;
                let y = d2 as usize;
                self.v_register[x] ^= self.v_register[y];
            }
            (8, _, _, 4) => {
                let x = d2 as usize;
                let y = d3 as usize;

                let (result, carry) = self.v_register[x].overflowing_add(self.v_register[y]);
                self.v_register[x] = result;
                self.v_register[0xF] = if carry { 1 } else { 0 };
            }
            (8, _, _, 5) => {
                let x = d2 as usize;
                let y = d3 as usize;

                let (res, borrow) = self.v_register[x].overflowing_sub(self.v_register[y]);

                self.v_register[x] = res;
                self.v_register[0xF] = if borrow { 0 } else { 1 };
            }
            (8, _, _, 6) => {
                let x = d2 as usize;
                let lsb = self.v_register[x] & 1;
                self.v_register[x] >>= 1;
                self.v_register[0xF] = lsb;
            }
            (8, _, _, 7) => {
                let x = d2 as usize;
                let y = d2 as usize;
                let (res, borrow) = self.v_register[y].overflowing_sub(self.v_register[x]);

                self.v_register[x] = res;
                self.v_register[0xF] = if borrow { 0 } else { 1 };
            }
            (8, _, _, 0xE) => {
                let x = d2 as usize;

                let msb = (self.v_register[x] >> 7) & 1;
                self.v_register[x] <<= 1;
                self.v_register[0xF] = msb;
            }
            (9, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;

                if self.v_register[x] != self.v_register[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.i_register = nnn;
            }
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;

                self.pc = (self.v_register[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                let x = d2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();

                self.v_register[x] = rng & nn;
            }
            (0xD, _, _, _) => {
                let x_cord = self.v_register[d2 as usize] as u16;
                let y_cord = self.v_register[d3 as usize] as u16;

                let num_rows = d4;

                let mut flipped = false;
                for y_line in 0..num_rows {
                    let addr = self.i_register + y_line as u16;
                    let pixeles = self.memory[addr as usize];
                    for x_line in 0..8 {
                        if (pixeles & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_cord + x_line) as usize % DISPLAY_WIDTH;
                            let y = (y_cord + y_line) as usize % DISPLAY_HEIGHT;

                            let idx = x + DISPLAY_WIDTH * y;
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                if flipped {
                    self.v_register[0xF] = 1;
                } else {
                    self.v_register[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => {
                let x = d2 as usize;
                let vx = self.v_register[x];
                let key = self.keypad[vx as usize];
                if key {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA ,1) => {
                let x = d2 as usize ;
                let vx = self.v_register[x];
                let key = self.keypad[vx as usize];
                if !key {
                    self.pc +=2 ;

                }
            }
            (0xF, _ , 0,7) => {
                let x = d2 as usize;
                self.v_register[x] = self.delay_timer ;
                
                
            }
            (0xF,_,0,0xA) => {
                let x = d2 as usize ;
                let mut pressed = false;
                for i in 0..self.keypad.len(){
                    if self.keypad[i]{
                        self.v_register[x] = i as u8;
                        pressed = false;
                        break;
                    }
                }
                if!pressed{
                    self.pc -= 2;
                }
            }
            // FX15 Delay Timer = VX
            (0xF, _ , 1 ,5) => {
                let x = d2 as usize;
                self.delay_timer = self.v_register[x];

            }
            // FX18 Sound Timer = VX

            (0xF, _ ,1,8) => {
                let x = d2 as usize;
                self.sound_timer = self.v_register[x];
            }

            // FX1E I += VX
            (0xF,_,1,0xE) => {
                let vx = self.v_register[d2 as usize];
                self.i_register = self.i_register.wrapping_add(vx);
            }
            // FX29 Set I to address of font character in VX
            (0xF, _ ,2,9) => {
                let x = d2 as usize;
                let c = self.v_register[x] as u16;
                self.i_register = c * 5 ;


            }
            // FX33 Stores BCD encoding of VX into I
            (0xF,_,3,3) => {
                let x = d2 as usize ;
                let vx = self.v_register[x] ;
                let h = vx / 100 ;
                let t = (vx/10)%10 ;
                let o = vx % 10 ;

                self.memory[self.i_register as usize] = h;
                self.memory[(self.i_register +1 ) as usize] = t;
                self.memory[(self.i_register +2) as usize] = o;

             }
            //  FX55 - Store V0 - VX into I
            (0xF , _,5,5) => {
               let x = d2 as usize;

               for i in 0..=x {
                self.memory[(self.i_register + (i as u16)) as usize] = self.v_register[i];
               } 
            }
            // FX65 Fills V0 thru VX with RAM values starting
            // at address in I
            (0xF , _ , 6,5) => {
                let x = d2 as usize ;
                let i = self.i_register as usize;

                for idx in 0..=x{
                    self.v_register[idx] = self.memory[i + idx];
                }
            }
            (_, _, _, _) => {
                unimplemented!(" Unimplemented op {}", op);
            }
        }
       
    }
}
