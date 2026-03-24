_STACK_SIZE = DEFINED(_STACK_SIZE) ? _STACK_SIZE : 0x2000;
_HEAP_SIZE = DEFINED(_HEAP_SIZE) ? _HEAP_SIZE : 1024;

_ABORT_STACK_SIZE = DEFINED(_ABORT_STACK_SIZE) ? _ABORT_STACK_SIZE : 256;
_SUPERVISOR_STACK_SIZE = DEFINED(_SUPERVISOR_STACK_SIZE) ? _SUPERVISOR_STACK_SIZE : 256;
_IRQ_STACK_SIZE = DEFINED(_IRQ_STACK_SIZE) ? _IRQ_STACK_SIZE : 256;
_FIQ_STACK_SIZE = DEFINED(_FIQ_STACK_SIZE) ? _FIQ_STACK_SIZE : 256;
_UNDEF_STACK_SIZE = DEFINED(_UNDEF_STACK_SIZE) ? _UNDEF_STACK_SIZE : 256;

MEMORY
{
   /* Address from Zynq Ultrascale+ Device TRM UG1085 */
   csu_rom_BASEADDR   : ORIGIN = 0xFFC00000, LENGTH = 0x0020000
   csu_ram_BASEADDR   : ORIGIN = 0xFFC40000, LENGTH = 0x0008000
   efuse_BASEADDR     : ORIGIN = 0xFFCC0000, LENGTH = 0x0010000
   pmu_rom_BASEADDR   : ORIGIN = 0xFFD00000, LENGTH = 0x0040000
   pmu_ram_BASEADDR   : ORIGIN = 0xFFDC0000, LENGTH = 0x0020000
   ocm_ram_0_BASEADDR : ORIGIN = 0xFFFC0000, LENGTH = 0x0010000
   ocm_ram_1_BASEADDR : ORIGIN = 0xFFFD0000, LENGTH = 0x0010000
   ocm_ram_2_BASEADDR : ORIGIN = 0xFFFE0000, LENGTH = 0x0010000
   ocm_ram_3_BASEADDR : ORIGIN = 0xFFFF0000, LENGTH = 0x0010000
}

ENTRY(_start)

SECTIONS
{
    /* Starts at 0x80000 (common for AArch64 boot) */
    . = 0x80000;

    .text : {
        /* Ensure your startup assembly/code is at the very beginning */
        KEEP(*(.text._start))
        *(.text .text.*)
    } > ocm_ram_0_BASEADDR

    .rodata : { *(.rodata .rodata.*) } > ocm_ram_0_BASEADDR
    .data : { *(.data .data.*) } > ocm_ram_0_BASEADDR
    
    .bss : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
        . = ALIGN(16);
    } > ocm_ram_0_BASEADDR

    /* Align the end of the binary to a page boundary */
    . = ALIGN(4096);
    __end = .;

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame) }
}