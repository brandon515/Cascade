ENTRY(start)

SECTIONS {
  . = 1M; /* start 1 megabyte into the address space */

  .boot :
  {
    *(.multiboot)
  }

  .text :
  {
    *(.text)
  }
}
