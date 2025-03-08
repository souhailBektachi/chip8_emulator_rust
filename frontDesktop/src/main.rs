use chip8_impl::*;
use std::env;
use sdl2::event::{self, Event};
use std::fs::File;
use std::io::Read;
use sdl2::pixels::Color;
use sdl2::render::Canvas;   
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 10;
use sdl2::keyboard::Keycode;

const TARGET_FPS: u32 = 60;
const FRAME_TIME: u32 = 1000 / TARGET_FPS; // Time per frame in milliseconds

const CPU_FREQ: f64 = 500.0; // Hz
const TIMER_FREQ: f64 = 60.0; // Hz
const CPU_STEP: f64 = 1.0 / CPU_FREQ;
const TIMER_STEP: f64 = 1.0 / TIMER_FREQ;

fn keymap(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::A => Some(0x4),
        Keycode::Z => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::Q => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::W => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn draw_screen(canvas: &mut Canvas<Window>, chip8: &Emulator) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    let screen_buf = chip8.get_display();
    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            if screen_buf[y * DISPLAY_WIDTH + x] {
                canvas.fill_rect(Rect::new(
                    x as i32, 
                    y as i32, 
                    1, 
                    1
                )).unwrap();
            }
        }
    }
    canvas.present();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Chip8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()  
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .present_vsync()  
        .build()
        .unwrap();
    
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        }
    }).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut chip8 = Emulator::new();
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("failed to read file");
    chip8.load(&buffer);

    let mut cpu_timer = 0.0;
    let mut delay_timer = 0.0;
    let mut last_time = std::time::Instant::now();

    'gameloop: loop {
        let current_time = std::time::Instant::now();
        let dt = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;

        cpu_timer += dt;
        while cpu_timer >= CPU_STEP {
            chip8.tick();
            cpu_timer -= CPU_STEP;
        }

        delay_timer += dt;
        while delay_timer >= TIMER_STEP {
            chip8.tick_timer();
            if chip8.sound_timer() > 0 {
                audio_device.resume();
            } else {
                audio_device.pause();
            }
            delay_timer -= TIMER_STEP;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode:Some(Keycode::Escape),..}  => break 'gameloop,
                Event::KeyDown {keycode: Some(key), ..} => {
                    if let Some(k) = keymap(key) {
                        chip8.set_key(k, true);
                    }
                },
                Event::KeyUp {keycode: Some(key), ..} => {
                    if let Some(k) = keymap(key) {
                        chip8.set_key(k, false);
                    }
                },
                _ => {}
            }

        }
        if(chip8.draw_flag){
            chip8.draw_flag = false;
            draw_screen(&mut canvas, &chip8);
        }

    
    }

    canvas.clear();
    canvas.present();

}
