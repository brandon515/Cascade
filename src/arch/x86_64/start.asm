global long_mode_start
global get_heap_start

section .text
bits 64
extern kmain
long_mode_start:
  mov rsp, stack_top ; ESP is the register with the stack pointer, just set it to the highest memory address because the stack goes down
  mov rbp, stack_top
  call kmain ; we've spent enough time in assembly, back to C
  ;mov rax, 0x2f592f412f4b2f4f
  ;mov qword [0xb8000], rax

get_heap_start:
  mov rax, heap_start
  ret

get_page_table:
  mov rax, cr3
  ret

section .bss
heap_start:
  resb 0x8000 ; 32 kilobytes
stack_bottom:
  resb 0x8000 ; 32 kilobytes
stack_top:
