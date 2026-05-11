#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod vga_buffer;
mod gdt;
mod interrupts;
mod memory;
mod allocator;
mod task;
mod syscall;
mod sync;
mod framebuffer;

use core::panic::PanicInfo;
use x86_64::VirtAddr;
use task::{Task, executor::Executor, shell, user};

#[no_mangle]
pub extern "C" fn _start_rust(multiboot_info_ptr: usize) -> ! {
    println!("Welcome to Scratch-OS!");

    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // Initialize memory and allocator
    {
        let boot_info = unsafe { &*(multiboot_info_ptr as *const multiboot2::BootInformation) };
        let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

        let mut frame_allocator = unsafe {
            memory::BootInfoFrameAllocator::init(
                memory_map_tag,
                boot_info.start_address(),
                boot_info.end_address()
            )
        };

        let mut mapper = unsafe { memory::init(VirtAddr::new(0)) };

        allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");

        if let Some(fb_tag_result) = boot_info.framebuffer_tag() {
            if let Ok(fb_tag) = fb_tag_result {
                let mut fb = framebuffer::Framebuffer::new(
                    fb_tag.address(),
                    fb_tag.width() as usize,
                    fb_tag.height() as usize,
                    fb_tag.pitch() as usize,
                );
                fb.clear(0x003366);
            }
        }
    }

    syscall::init();

    println!("Kernel initialized. Transitions to User-Mode...");

    let mut executor = Executor::new();
    executor.spawn(Task::new(async {
        user::jump_to_user_mode();
    }));

    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
