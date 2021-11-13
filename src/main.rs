//main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";




// Function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

//defines the entry interface
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}