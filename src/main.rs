#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!{}", "!");
    panic!("OH SHIT");
    loop {}
}

// fn main() {
//     println!("Hello, world!");
// }
