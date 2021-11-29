global outb
global inb

section .text
bits 64

outb:
  mov edx, edi
  mov ax, si
  out dx, al ; the port(dx) is 16 bits and the data(al) is 8 bits
  ret

inb:
  mov dx, di
  in ax, dx 
  ret
