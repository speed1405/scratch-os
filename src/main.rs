#![no_std]
#![no_main]

use core::panic::PanicInfo;

fn hlt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hlt_loop()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop()
}
