global long_mode_start
global get_page_table
extern gdt64

section .text
bits 64
extern kernel_main
long_mode_start:
  cli
  mov ax, 0
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax
  call kernel_main
  ;mov rax, 0x2f592f412f4b2f4f
  ;mov qword [0xb8000], rax
  hlt

get_page_table:
  mov rax, cr3 ; move the address of the page table to the return register
  ret
