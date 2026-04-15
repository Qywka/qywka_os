#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod command_index;
mod commands;
mod interrupts;
mod vga_buffer;

use core::panic::PanicInfo;
use spin::Mutex;

struct ShellBuffer {
    data: [u8; 64],
    pos: usize,
}

static SHELL_BUF: Mutex<ShellBuffer> = Mutex::new(ShellBuffer {
    data: [0; 64],
    pos: 0,
});

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::clear_screen();
    println!("Welcome to QwexOS!");
    print!("> ");

    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    loop {
        x86_64::instructions::hlt();
    }
}

pub fn handle_keyboard_input(key: u8) {
    let mut buf = SHELL_BUF.lock();

    match key {
        b'\n' => {
            println!("");
            if let Ok(s) = core::str::from_utf8(&buf.data[..buf.pos]) {
                command_index::execute(s);
            }
            buf.pos = 0;
            print!("> ");
        }
        8 | 127 => {
            if buf.pos > 0 {
                buf.pos -= 1;
                vga_buffer::backspace();
            }
        }
        byte => {
            if buf.pos < buf.data.len() {
                let current_pos = buf.pos;
                buf.data[current_pos] = byte;
                buf.pos += 1;
                print!("{}", byte as char);
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
