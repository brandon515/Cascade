global start
global check_multiboot
global check_cpuid
global check_long_mode
global enable_paging
global enable_long_mode
global error
global gdt64
extern bootstrap
extern long_mode_start

section .text
bits 32 ; 32 bit instructions, this will change to 64 once we go to long mode
; 0xb8000 is the VGA buffer for the screen
start:
  cli
  mov esp, stack_top ; ESP is the register with the stack pointer, just set it to the highest memory address because the stack goes down
  mov ebp, stack_top
  mov edi, ebx ; move the pointer to the memory_map to the first parameter
  mov esi, eax ; move the magic number that says we're in multiboot to the second param
  call  bootstrap 
  mov dword [0xb8000], 0x2f4b2f4f ; put OK on the screen
  hlt

error: ; in x86_64 arch rdi is the register that has the first parameter
  and di, 0x00ff ; get the first byte of the first parameter
  add di, 0x4f00 ; make the background red and the text white
  mov dword [0xb8000], 0x4f524f45 ; ER
  mov dword [0xb8004], 0x4f3a4f52 ; R:
  mov word [0xb8008], 0x4f00 ;   
  mov word [0xb800a], di ; register with the error code
  hlt

check_cpuid:
  pushfd ; push the FLAGS onto the stack and go up the stack
  pop eax ; get the FLAGS
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

enable_paging: ; TODO this is hard coded as hell, make it go off a single constant
  mov edi, 0x1000   ; the location in memory of the 4th level page table mov cr3, edi      ; let the CPU know where the page table is
  mov cr3, edi
  xor eax, eax      ; zero out eax
  mov ecx, 4096
  rep stosd         ; this instruction puts the value of EAX into the memory pointed at by EDI ECX number of times
  mov edi, cr3      ; makes sure EDI is pointing to the begining of the page table
  mov DWORD [edi], 0x2003   ; set the first page table to 0x2003, 3 is to set page to present and writable
  add edi, 0x1000
  mov DWORD [edi], 0x3003   ; doing the same thing for the next 3 levels
  add edi, 0x1000
  mov DWORD [edi], 0x4003   ; 0x4000 is the level 1 page table
  add edi, 0x1000
  mov ebx, 0x00000003       ; set the first two bits to present and writable
  mov ecx, 512              ; ecx controls the amount of time the loop instruction loops

.set_entry: ; Identity make the first 512 pages, which is  2 MiB of ram
  mov DWORD [edi], ebx      ; set the page to present and writable
  add ebx, 0x1000           ; increase the physical memory we're mapping by 4096 or 4KiB
  add edi, 8                ; add the page table index by 8 bytes which is the size of a 64-bit memory address
  loop .set_entry

  mov eax, cr4    ; set the a register to CR4
  or eax, 1 << 5  ; set the 5th bit to enable physical address extension for paging
  mov cr4, eax    ; push it back to CR4
  mov eax, 0x1000
  ret

enable_long_mode:
  mov ecx, 0xC0000080
  rdmsr               ; read from model-specific register or MSR into EAX
  or eax,0100000000b  ; enable the 8th bit which shows that long mode is enabled
  wrmsr               ; write back to MSR from EAX
  mov eax, cr0
  or eax, 0x80000001  ; protected bit (0) and paging bit (8)
  mov cr0, eax
  lgdt [gdt64.pointer]
  cli
  mov ax, gdt64.data
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax
  mov edx, gdt64 ; give C the gdt table address
  jmp gdt64.code:long_mode_start

gdt64:
.null: equ $ - gdt64
  dq 0 ; zero entry, required to be the first entry in the GDT
.code: equ $ - gdt64 ; set the code segment label to the offset instead of the raw address
  dw 0xFFFF         ; limit low
  dw 0              ; base low
  db 0              ; base middle
  db 10011010b      ; access byte
  db 10101111b      ; from left to right: 4 kib blocks (pages) size (must be 0 in 64) L (64 bit mode)
  db 0              ; base high
.data: equ $ - gdt64 ; set the data segment label
  dw 0xFFFF         ; limit low
  dw 0              ; base low
  db 0              ; base middle
  db 10010010b      ; access byte
  db 00001111b      ; from left to right: 4 kib blocks (pages) size (must be 0 in 64) L (64 bit mode)
  db 0              ; base high
  times 0x100 db 0  ; 8*32 so 32 entries
.pointer: ; this structure is looked for by lgdt
  dw $ - gdt64 - 1 ; GDT length
  dq gdt64  ; GDT address

section .bss
stack_bottom:
  resb 0x2000 ; 8 kilobytes
stack_top:

