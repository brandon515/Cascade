section .multiboot
extern start
extern start_end
extern boot_stack_bottom
bits 32
header_start:
  dd 0xE85250D6                 ; Magic number for multiboot2 stanadard
  dd 0                          ; Architecture 0, which is protected mode i386
  dd header_end - header_start  ; The length of this section
  ; checksum required by the standard
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
  ; 0x100000000 is so the compiler doesn't yell at us

  ; multiboot tags go here

entry_address_tag_start:
  dw 3 ; #define MULTIBOOT_HEADER_TAG_ENTRY_ADDRESS  3
  dw 1 ; #define MULTIBOOT_HEADER_TAG_OPTIONAL 1
  dd entry_address_tag_end-entry_address_tag_start ; size
  dd start ; where the bootloader should jump to
entry_address_tag_end:
align 8
;framebuffer_tag_start:
;  dw 5 ; #define MULTIBOOT_HEADER_TAG_FRAMEBUFFER  5
;  dw 1 ; #define MULTIBOOT_HEADER_TAG_OPTIONAL 1
;  dd framebuffer_tag_end - framebuffer_tag_start ; size
;  dd 800 ; Width
;  dd 600 ; height
;  dd 32 ; depth
;framebuffer_tag_end:
align 8
  ; end tags
  dw 0  ; type has to be 0
  dw 0  ; just deliniating that the tags are over
  dd 8  ; size has to be 8
header_end:
