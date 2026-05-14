#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::clear();
    vga::print_at(b"NODDY", 0, 0, vga::Color::LightGreen);
    vga::print_at(b"Not Obviously Doing Diddly Yet", 0, 1, vga::Color::LightGray);
    vga::print_at(b"v0.1.0", 0, 3, vga::Color::DarkGray);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
