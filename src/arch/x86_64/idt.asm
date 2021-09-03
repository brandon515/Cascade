global load_idt

section .text
bits 64
load_idt:
  lidt [rdi]
  ret
