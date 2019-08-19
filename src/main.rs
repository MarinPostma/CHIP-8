pub mod cpu;
pub mod debugger;
pub mod rom_mgr;

use cpu::Tick;
use std::env::args;
use v_display::display::DisplayBuilder;
use v_display::sdl2::event::Event;
use v_display::sdl2::keyboard::Keycode;

//fn main() {
//    let filename = args()
//        .into_iter()
//        .nth(1)
//        .expect("A ROM file should be provided as argument");
//    let mut cpu = cpu::CPU::new();
//    let rom = rom_mgr::RomMgr::new(&filename);
//    let mut display = DisplayBuilder::new(&filename, 64, 32, 10)
//        .with_margin(5, 5)
//        .build()
//        .unwrap();
//    cpu.load_rom(rom);
//    let mut cpu = debugger::Debugger::new(cpu);
//    'main: loop {
//        for event in display.get_event_pump().poll_iter() {
//            use Keycode::*;
//            match event {
//                Event::KeyDown {
//                    keycode: Some(Escape),
//                    ..
//                } => {
//                    break 'main;
//                }
//                Event::KeyDown {
//                    keycode: Some(key), ..
//                } => cpu.cpu.set_key_down(key, true),
//                Event::KeyUp {
//                    keycode: Some(key), ..
//                } => cpu.cpu.set_key_down(key, false),
//                _ => {}
//            }
//        }
//        cpu.tick();
//        if cpu.cpu.draw {
//            cpu.cpu.draw = false;
//            display.from_buffer(&cpu.cpu.vram);
//            display.refresh();
//        }
//        std::thread::sleep(std::time::Duration::from_millis(150));
//    }
//}
fn main() {
    let filename = args()
        .into_iter()
        .nth(1)
        .expect("A ROM file should be provided as argument");
    let mut cpu = cpu::CPU::new();
    let rom = rom_mgr::RomMgr::new(&filename);
    let mut display = DisplayBuilder::new(&filename, 64, 32, 10)
        .with_margin(5, 5)
        .build()
        .unwrap();
    cpu.load_rom(rom);
    'main: loop {
        for event in display.get_event_pump().poll_iter() {
            use Keycode::*;
            match event {
                Event::KeyDown {
                    keycode: Some(Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => cpu.set_key_down(key, true),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => cpu.set_key_down(key, false),
                _ => {}
            }
        }
        cpu.tick();
        if cpu.draw {
            cpu.draw = false;
            display.from_buffer(&cpu.vram);
            display.refresh();
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}
//fn main() {
//    let mut cpu = cpu::CPU::new();
//    let mut display = DisplayBuilder::new("hello", 64, 32, 10)
//        .with_margin(5, 5)
//        .build()
//        .unwrap();
//    cpu.ram[0x200] = 0x20
//    'main: loop {
//        for event in display.get_event_pump().poll_iter() {
//            use Keycode::*;
//            match event {
//                Event::KeyDown {
//                    keycode: Some(Escape),
//                    ..
//                } => {
//                    break 'main;
//                }
//                Event::KeyDown {
//                    keycode: Some(key), ..
//                } => cpu.set_key_down(key),
//                _ => {}
//            }
//        }
//        cpu.tick();
//        display.from_buffer(&cpu.vram);
//        display.refresh();
//        std::thread::sleep(std::time::Duration::from_millis(50));
//    }
//}
