global start

section .text
bits 32 ; 32 bit instructions, this will change to 64 once we go to long mode
; 0xb8000 is the VGA buffer for the screen
start:
  mov esp, stack_top ; ESP is the register with the stack pointer, just set it to the highest memory address because the stack goes down
  call check_multiboot
  call check_cpuid
  call check_long_mode
  call enable_paging
  mov dword [0xb8000], 0x2f4b2f4f ; put OK on the screen
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
check_cpuid:
  pushfd ; push the FLAGS into EAX and go up the stack
  pop eax
  mov ecx, eax
  xor eax, 1 << 21 ; if we can flip this bit CPUID is enabled
  push eax
  popfd
  pushfd ; copy the FLAGS back into EAX, to see if the bit was flipped
  pop eax
  push ecx
  popfd
  xor eax, ecx
  jz .no_cpuid ; two values xor'd together should be zero if they're the same
  ret
.no_cpuid:
  mov al, "1"
  jmp error
check_long_mode: ; check to see if we can enter long mode
  mov eax, 0x80000000 ; get highest extended function implemented
  cpuid
  cmp eax, 0x80000001
  jb .no_long_mode ; if the return value is less that 0x8000000 then that means there are no extended functions and long mode is impossible
  mov eax, 0x80000001
  cpuid
  test edx, 1 << 29 ; fancy for an AND operation, we're just checking the 29th bit which is 1 if long mode is possible on this cpu
  jz .no_long_mode
  ret
.no_long_mode:
  mov al, "2"
  jmp error
enable_paging:
  mov edi, 0x1000   ; the location in memory of the 4th level page table
  mov cr3, edi      ; let the CPU know where the page table is
  xor eax, eax      ; zero out eax
  mov ecx, 4096
  rep stosd         ; this instruction puts the value of EAX into the memory pointed at by EDI ECX number of times
  mov edi, cr3      ; makes sure EDI is pointing to the begining of the page table
  mov DWORD [edi], 0x2003   ; set the point pointed at by EDI to 0x2003, 3 is to set page to present and writable
  add edi, 0x1000
  mov DWORD [edi], 0x3003   ; doing the same thing for the next 3 levels
  add edi, 0x1000
  mov DWORD [edi], 0x4003   ; 0x4000 is the level 1 page table
  add edi, 0x1000
  mov ebx, 0x00000003       ; set the first two bits to present and writable
  mov ecx, 512              ; ecx controls the amount of time the loop instruction loops

.set_entry:
  mov DWORD [edi], ebx      ; set the page to present and writable
  add ebx, 0x1000           ; increase the physical memory we're mapping by 4096 or 4KiB
  add edi, 8                ; add the page table index by 8 bytes which is the size of a 64-bit memory address
  loop .set_entry

  mov eax, cr4    ; set the a register to CR4
  or eax, 1 << 5  ; set the 5th bit to enable paging
  mov cr4, eax    ; push it back to CR4
  ret
enable_long_mode:
  mov eax, cr4
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
section .bss
stack_bottom:
  resb 64
stack_top:
