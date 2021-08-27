global start

section .text
bits 32 ; 32 bit instructions, this will change to 64 once we go to long mode
; 0xb8000 is the VGA buffer for the screen
start:
  mov esp, stack_top ; ESP is the register with the stack pointer, just set it to the highest memory address because the stack goes down
  mov dword [0xb8000], 0x2f4b2f4f
  hlt
error:
  mov dword [0xb8000], 0x4f524f45 ; ER
  mov dword [0xb8000], 0x4f524f45 ; R:
  mov dword [0xb8000], 0x4f524f45 ;   
  mov byte [0xb8000], al ; register with the error code
  hlt
check_multiboot:
  cmp eax, 0x36d76289 ; according to the multiboot spec this must be written to eax when the kernel is loaded
  jne .no_multiboot ; jump to multiboot if the previous comparison isn't equal
  ret
.no_multiboot:
  mov al, "0"
  jmp error

section .bss
stack_bottom:
  resb 64
stack_top:
