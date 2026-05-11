.section .boot, "ax"
.code32

# Multiboot2 Header
.align 8
multiboot_header:
    .long 0xe85250d6                # magic
    .long 0                         # architecture 0 (protected mode i386)
    .long multiboot_header_end - multiboot_header # header length
    .long -(0xe85250d6 + 0 + (multiboot_header_end - multiboot_header)) # checksum

    # End tag
    .short 0
    .short 0
    .long 8
multiboot_header_end:

.global _start
_start:
    cli
    mov $stack_top, %esp

    # Check multiboot magic
    cmp $0x36d76289, %eax
    jne .no_multiboot

    # Save multiboot info pointer
    mov %ebx, %edi

    call check_cpuid
    call check_long_mode

    call setup_page_tables
    call enable_paging

    # Load 64-bit GDT
    lgdt (gdt64_pointer)
    ljmp $8, $long_mode_start

.no_multiboot:
    movl $0x4f524f45, (0xb8000) # "ER" in red
    hlt

check_cpuid:
    # Check if CPUID is supported by attempting to flip the ID bit (21) in EFLAGS
    pushfl
    pop %eax
    mov %eax, %ecx
    xor $(1 << 21), %eax
    push %eax
    popfl
    pushfl
    pop %eax
    push %ecx
    popfl
    cmp %eax, %ecx
    je .no_cpuid
    ret
.no_cpuid:
    movl $0x4f314f45, (0xb8000) # "E1"
    hlt

check_long_mode:
    mov $0x80000000, %eax
    cpuid
    cmp $0x80000001, %eax
    jb .no_long_mode
    mov $0x80000001, %eax
    cpuid
    test $(1 << 29), %edx
    jz .no_long_mode
    ret
.no_long_mode:
    movl $0x4f324f45, (0xb8000) # "E2"
    hlt

setup_page_tables:
    # Identity map the first 1GB using 2MB pages
    # P4 -> P3 -> P2
    mov $p3_table, %eax
    or $0b11, %eax # present + writable
    mov %eax, (p4_table)

    mov $p2_table, %eax
    or $0b11, %eax
    mov %eax, (p3_table)

    # Map each P2 entry to a huge 2MB page
    mov $0, %ecx
.map_p2_table:
    mov $0x200000, %eax # 2MB
    mul %ecx
    or $0b10000011, %eax # present + writable + huge
    mov %eax, p2_table(, %ecx, 8)

    inc %ecx
    cmp $512, %ecx
    jne .map_p2_table
    ret

enable_paging:
    # Pass P4 table to CR3
    mov $p4_table, %eax
    mov %eax, %cr3

    # Enable PAE
    mov %cr4, %eax
    or $(1 << 5), %eax
    mov %eax, %cr4

    # Set Long Mode bit in EFER MSR
    mov $0xC0000080, %ecx
    rdmsr
    or $(1 << 8), %eax
    wrmsr

    # Enable paging in CR0
    mov %cr0, %eax
    or $(1 << 31), %eax
    mov %eax, %cr0
    ret

.code64
long_mode_start:
    # Load 0 into segment registers
    mov $0, %ax
    mov %ax, %ss
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    # EDI already contains multiboot pointer from _start
    # In 64-bit calling convention, RDI is the first arg.
    # Since we are in long mode, we should ensure the upper bits of RDI are clear.
    mov %edi, %edi

    # Call _start_rust
    call _start_rust
    hlt

.section .bss
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 4096 * 4
stack_top:

.section .rodata
gdt64:
    .quad 0 # zero entry
    # Code segment: descriptor type (1), present (1), read/write (1),
    # executable (1), 64-bit (1).
    .quad (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
gdt64_pointer:
    .short . - gdt64 - 1
    .quad gdt64
