global long_mode_start

section .text
bits 64
extern kmain
long_mode_start:
  mov rsp, stack_top ; ESP is the register with the stack pointer, just set it to the highest memory address because the stack goes down
  mov rbp, stack_top
  call kmain
  ;mov rax, 0x2f592f412f4b2f4f
  ;mov qword [0xb8000], rax

section .bss
stack_bottom:
  resb 0x8000 ; 32 kilobytes
stack_top:
