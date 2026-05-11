use x86_64::registers::model_specific::{LStar, SFMask, Efer, EferFlags, GsBase, KernelGsBase};
use x86_64::registers::model_specific::Msr;
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;
use crate::gdt;

#[repr(C)]
struct CpuContext {
    kernel_stack: u64,
    user_stack: u64,
}

pub fn init() {
    unsafe {
        // Enable syscall/sysret
        Efer::update(|flags| flags.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));

        // Set entry point
        LStar::write(VirtAddr::new(syscall_handler as *const () as u64));

        // Mask interrupts on syscall
        SFMask::write(RFlags::INTERRUPT_FLAG);

        // Map segment selectors for syscall/sysret
        let selectors = &gdt::GDT.1;
        let star_val = ((selectors.user_code_selector.0 as u64 - 16) << 48) |
                       ((selectors.kernel_code_selector.0 as u64) << 32);

        Msr::new(0xC0000081).write(star_val);

        // Initialize GS Base for stack swapping using the TSS RSP0
        static mut CONTEXT: CpuContext = CpuContext { kernel_stack: 0, user_stack: 0 };
        CONTEXT.kernel_stack = gdt::TSS.privilege_stack_table[0].as_u64();

        GsBase::write(VirtAddr::from_ptr(&raw const CONTEXT));
        KernelGsBase::write(VirtAddr::from_ptr(&raw const CONTEXT));
    }
}

#[no_mangle]
pub extern "C" fn syscall_handler_rust(syscall_id: u64, arg1: u64, arg2: u64) -> u64 {
    match syscall_id {
        1 => { // sys_write
            let s = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(arg1 as *const u8, arg2 as usize)) };
            crate::print!("{}", s);
            0
        },
        _ => 0,
    }
}

#[unsafe(naked)]
unsafe extern "C" fn syscall_handler() {
    core::arch::naked_asm!(
        "swapgs",
        "mov gs:8, rsp",     // save user rsp
        "mov rsp, gs:0",     // load kernel rsp
        "push r11",
        "push rcx",

        "mov rdx, rsi",
        "mov rsi, rdi",
        "mov rdi, rax",

        "call syscall_handler_rust",

        "pop rcx",
        "pop r11",
        "mov rsp, gs:8",
        "swapgs",
        "sysretq"
    );
}
