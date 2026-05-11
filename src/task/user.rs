use x86_64::VirtAddr;
use crate::gdt;

#[no_mangle]
pub extern "C" fn jump_to_user_mode() -> ! {
    let selectors = &gdt::GDT.1;

    const STACK_SIZE: usize = 4096 * 2;
    static mut USER_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let stack_top = VirtAddr::from_ptr(unsafe { &raw const USER_STACK }) + STACK_SIZE;

    unsafe {
        core::arch::asm!(
            "push rdx",     // user data segment
            "push rax",     // user stack pointer
            "pushfq",       // rflags
            "push rdi",     // user code segment
            "push rsi",     // instruction pointer
            "iretq",
            in("rdi") selectors.user_code_selector.0 | 3,
            in("rsi") user_entry as *const () as u64,
            in("rax") stack_top.as_u64(),
            in("rdx") selectors.user_data_selector.0 | 3,
            options(noreturn)
        );
    }
}

extern "C" fn user_main() -> ! {
    let msg = "Welcome to Scratch-OS User-Mode Shell!\n> ";
    syscall_write(msg);

    loop {
        let help_msg = "User-mode execution is active. Syscalls are working.\n> ";
        syscall_write(help_msg);

        for _ in 0..100000000 { unsafe { core::arch::asm!("nop") } }
    }
}

fn syscall_write(s: &str) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1u64, // sys_write
            in("rdi") s.as_ptr() as u64,
            in("rsi") s.len() as u64,
            out("rcx") _,
            out("r11") _,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_entry() -> ! {
    user_main();
}
