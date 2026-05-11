ENTRY(_start)

SECTIONS {
    . = 1M;
    __kernel_start = .;

    .boot :
    {
        KEEP(*(.boot))
    }

    .text :
    {
        *(.text)
    }

    .rodata :
    {
        *(.rodata)
    }

    .data :
    {
        *(.data)
    }

    .bss :
    {
        *(.bss)
    }

    __kernel_end = .;
}
