pub mod chip8;
pub mod config;
pub mod cpu;
pub mod debugger;

use clap::{App, Arg};
use config::*;
use v_display::display::DisplayBuilder;

fn main() {
    let matches = App::new("CHIP-8 emu")
        .version("0.1")
        .author("mpostma")
        .about("A basic chip-8 emulator")
        .arg(
            Arg::with_name("ROM")
                .help("path to the rom to emulate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("print debug info"),
        )
        .arg(
            Arg::with_name("clock_time")
                .short("c")
                .long("clock")
                .takes_value(true)
                .help("set clock time value in ms (default to 2ms)"),
        )
        .get_matches();
    //safe to unwrap here because ROM is required.
    let filename = matches.value_of("ROM").unwrap();

    let display = DisplayBuilder::new(
        &filename,
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
        PIX_SIZE as u32,
    )
    .with_margin(5, 5)
    .build()
    .unwrap();

    let cpu = cpu::CPU::new();
    let clock_time = matches
        .value_of("clock_time")
        .unwrap_or("2")
        .parse::<u64>()
        .expect("invalid clock value");

    // should be handled with polymorphism, but it's complicated...
    match matches.occurrences_of("debug") {
        1 => {
            let mut chip8 = chip8::Chip8::new(display, debugger::Debugger::new(cpu));
            chip8.load(&filename);
            chip8.run(clock_time);
        }
        _ => {
            let mut chip8 = chip8::Chip8::new(display, cpu);
            chip8.load(&filename);
            chip8.run(clock_time);
        }
    }
}
