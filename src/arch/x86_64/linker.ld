ENTRY(start)

SECTIONS {
  . = 1M; /* start 1 megabyte into the address space */

  .boot : ALIGN(0x1000)
  {
   *(.multiboot)
  }

  .text : ALIGN(0x1000)
  {
    *(.text)
    *(.rodata)
  }
  .bss : ALIGN(0x1000)
  {
    *(.bss)
  }
}
