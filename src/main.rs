#![no_std]
#![no_main]

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;

entry_point!(kernel_main);

static HELLO: &[u8] = b"Hello World!";

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    loop {}
}
