global long_mode_start

section .text
bits 64
long_mode_start:
  cli
  mov ax, 0
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax
  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8000], rax
  hlt
