# Scratch-OS Roadmap

This document outlines the development plan for a monolithic x86_64 operating system written in Rust.

## Vision
A hobbyist operating system featuring a monolithic kernel, user-mode applications, a full shell, and eventually a desktop environment.

## Technical Stack
- **Architecture:** x86_64
- **Language:** Rust (`no_std`, `no_main`)
- **Bootloader:** GRUB (via Multiboot2)
- **Emulation:** QEMU
- **Build System:** Cargo with custom target specifications

---

## Phase 1: Foundation & Bootstrapping
*Goal: Boot into a Rust kernel and display "Hello World".*

- [ ] **Environment Setup:**
    - Configure Rust nightly toolchain.
    - Create `x86_64-unknown-none.json` target specification.
    - Set up `Cargo.toml` with `bootloader` or custom assembly stub.
- [ ] **Multiboot2 Header:**
    - Implement a Multiboot2 compliant header to allow GRUB to load the kernel.
- [ ] **Kernel Entry Point:**
    - Write the assembly stub to transition from 32-bit (GRUB) to 64-bit long mode.
    - Call into the Rust `_start` function.
- [ ] **VGA Text Buffer Driver:**
    - Create a basic driver to print strings to the screen (0xB8000).
    - Implement the `core::fmt::Write` trait for easy formatting.

## Phase 2: Interrupts & CPU Configuration
*Goal: Handle CPU exceptions and hardware interrupts.*

- [ ] **Global Descriptor Table (GDT):**
    - Set up a new GDT.
    - Include Task State Segment (TSS) for stack switching.
- [ ] **Interrupt Descriptor Table (IDT):**
    - Define handlers for CPU exceptions (Division by zero, Page Fault, etc.).
    - Implement a `breakpoint` exception for testing.
- [ ] **Programmable Interrupt Controller (PIC/APIC):**
    - Remap the PIC or initialize the APIC to handle hardware interrupts.
- [ ] **PS/2 Keyboard Driver:**
    - Handle keyboard interrupts and decode scancodes.

## Phase 3: Memory Management
*Goal: Dynamic memory allocation and virtual memory.*

- [ ] **Physical Memory Manager (PMM):**
    - Parse the memory map provided by Multiboot2.
    - Implement a Frame Allocator (Bitmap or Stack based).
- [ ] **Paging:**
    - Map the kernel into the higher half of the address space.
    - Implement functions to map/unmap pages dynamically.
- [ ] **Kernel Heap:**
    - Implement a `GlobalAlloc` to enable the `alloc` crate (`Box`, `Vec`, `String`).

## Phase 4: Multitasking & User-Mode
*Goal: Run multiple tasks and transition to ring 3.*

- [ ] **Kernel Threads:**
    - Implement a basic scheduler (Round Robin).
    - Context switching logic (saving/restoring registers).
- [ ] **User-Mode Transition:**
    - Set up user-mode segments in the GDT.
    - Use `iretq` or `sysret` to jump to ring 3.
- [ ] **System Calls:**
    - Implement the `syscall` instruction handler.
    - Create a basic API for user-mode (e.g., `exit`, `print`).

## Phase 5: Filesystem & Shell
*Goal: Load programs from "disk" and interact via a shell.*

- [ ] **Initrd (Initial RAM Disk):**
    - Bundle a simple filesystem (like Tar) into the kernel image or load it as a GRUB module.
- [ ] **Virtual File System (VFS):**
    - Abstract file operations (`open`, `read`, `write`).
- [ ] **User-Mode Shell:**
    - Compile a separate Rust program as an ELF file.
    - Load and execute the ELF from the Initrd.
    - Implement basic commands (`ls`, `cat`, `help`).

## Phase 6: Graphics & Desktop
*Goal: Move beyond text mode to a graphical interface.*

- [ ] **Framebuffer Support:**
    - Request a linear framebuffer from GRUB (VBE/GOP).
    - Implement basic drawing primitives (pixels, rectangles).
- [ ] **Font Rendering:**
    - Load a PSF font and render text to the framebuffer.
- [ ] **Windowing System:**
    - Implement a compositor to manage multiple windows.
    - Handle mouse input (PS/2 or USB).

## Phase 7: SMP (Symmetric Multiprocessing)
*Goal: Utilize all CPU cores.*

- [ ] **APIC Initialization:**
    - Set up the Local APIC and I/O APIC.
- [ ] **AP Bootstrapping:**
    - Send Startup IPIs (SIPI) to bring up Application Processors.
- [ ] **Synchronization:**
    - Implement Spinlocks and Mutexes that are safe for multicore use.

---

## Future Directions
- **Networking:** VirtIO-net drivers and a TCP/IP stack.
- **USB:** EHCI/XHCI drivers for mouse/keyboard/storage.
- **Storage:** AHCI/NVMe drivers.
- **Porting:** Getting standard C/Rust libraries to run on Scratch-OS.
