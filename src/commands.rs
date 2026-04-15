use crate::command_index::COMMAND_REGISTRY;
use crate::println;
use crate::vga_buffer;

pub fn clear(_args: &str) {
    vga_buffer::clear_screen();
}

pub fn echo(args: &str) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args);
    }
}

pub fn help(_args: &str) {
    println!("Available commands:");
    for cmd in COMMAND_REGISTRY {
        println!("- {}", cmd.name);
    }
}

pub fn info(_args: &str) {
    println!("QwexOS v0.1 - Custom Rust Kernel");
    println!("Architecture: x86_64");
}
