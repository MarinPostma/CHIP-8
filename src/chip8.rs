use crate::cpu::Processor;
use std::fs::File;
use std::io::Read;
use v_display::display::Display;
use v_display::sdl2::event::Event;
use v_display::sdl2::keyboard::Keycode;

pub struct Chip8<T: Processor> {
    display: Display,
    cpu: T,
}

#[derive(PartialEq)]
pub enum State {
    Continue,
    Stop,
}

macro_rules! set_key {
    ($self:ident, $key:expr, $is_down: expr) => {
        match $key {
            X => $self.cpu.set_key_press(0, $is_down),
            Num1 => $self.cpu.set_key_press(1, $is_down),
            Num2 => $self.cpu.set_key_press(2, $is_down),
            Num3 => $self.cpu.set_key_press(3, $is_down),
            Q => $self.cpu.set_key_press(4, $is_down),
            W => $self.cpu.set_key_press(5, $is_down),
            E => $self.cpu.set_key_press(6, $is_down),
            A => $self.cpu.set_key_press(7, $is_down),
            S => $self.cpu.set_key_press(8, $is_down),
            D => $self.cpu.set_key_press(9, $is_down),
            Z => $self.cpu.set_key_press(10, $is_down),
            C => $self.cpu.set_key_press(11, $is_down),
            Num4 => $self.cpu.set_key_press(12, $is_down),
            R => $self.cpu.set_key_press(13, $is_down),
            F => $self.cpu.set_key_press(14, $is_down),
            V => $self.cpu.set_key_press(15, $is_down),
            _ => (),
        }
    };
}

impl<T> Chip8<T>
where
    T: Processor,
{
    pub fn new(display: Display, cpu: T) -> Self {
        Self {
            cpu: cpu,
            display: display,
        }
    }

    pub fn load(&mut self, filename: &str) {
        let mut file = File::open(&filename).expect("error while opening the file.");
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .expect("The ROM could not be read.");
        self.cpu.load_rom(&data)
    }

    pub fn run(&mut self) {
        let mut buffer: [(u8, u8, u8); 64 * 32] = [(0, 0, 0); 64 * 32];
        while self.send_key_event() == State::Continue {
            self.cpu.tick();
            if self.cpu.should_redraw() {
                self.cpu.drawn();
                self.cpu.get_vram_buffer(&mut buffer);
                self.display.from_buffer(&buffer);
                self.display.refresh();
            }
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }
    pub fn send_key_event(&mut self) -> State {
        for event in self.display.get_event_pump().poll_iter() {
            use Keycode::*;
            match event {
                Event::KeyDown {
                    keycode: Some(Escape),
                    ..
                } => {
                    return State::Stop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => set_key!(self, key, true),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => set_key!(self, key, false),
                _ => {}
            }
        }
        State::Continue
    }
}
