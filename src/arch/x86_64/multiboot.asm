section .multiboot
header_start:
  dd 0xe85250d6                 ; Magic number for multiboot2 stanadard
  dd 0                          ; Architecture 0, which is protected mode i386
  dd header_end - header_start  ; The length of this section
  ; checksum required by the standard
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
  ; 0x100000000 is so the compiler doesn't yell at us

  ; multiboot tags go here

  ; end tags
  dw 0  ;type
  dw 0  ;flags
  dd 8  ;size
header_end:
