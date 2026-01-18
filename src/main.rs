#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[macro_use]
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("52");
    loop {}
}
