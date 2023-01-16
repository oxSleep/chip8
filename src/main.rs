use std::{thread, time::Duration};

use chip8::Cpu;
mod driver;
mod font;


// TODO // implement audio.

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();

    let mut cpu = Cpu::default();
    let mut keypad_dri = driver::Keypad::new(&sdl_context);
    let mut display = driver::Display::new(&sdl_context);

    cpu.load_program();

    while let Ok(keypad) = keypad_dri.poll() {
        let out = cpu.cycle(keypad);

        if out.0 {
            display.draw(out.1);
        }

        // if output.beep {
        //     audio_driver.start_beep();
        // } else {
        //     audio_driver.stop_beep();
        // }

        thread::sleep(sleep_duration);
    }
}
