pub mod chip8;
pub mod config;
pub mod cpu;
pub mod debugger;

use config::*;
use std::env::args;
use v_display::display::DisplayBuilder;

fn main() {
    let filename = args()
        .into_iter()
        .nth(1)
        .expect("A ROM file should be provided as argument");
    let mut cpu = cpu::CPU::new();
    let mut display = DisplayBuilder::new(
        &filename,
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
        PIX_SIZE as u32,
    )
    .with_margin(5, 5)
    .build()
    .unwrap();
    let mut chip8 = chip8::Chip8::new(display, cpu);
    chip8.load(filename);
    chip8.run();
}
