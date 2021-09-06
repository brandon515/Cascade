#include <stdint.h>
#include "printf.h"
#include "gdt.h"
#include "../arch/x86_64/idt.h"
#include "interrupts.h"

typedef struct _memory_sector{
  uint32_t type;
  uint32_t size;
  uint32_t lower;
  uint32_t upper;
} memory_sector;


void kmain(memory_sector* info, uint64_t* page_table, gdt_entry* gdt){
  cls();
  init_idt();
  create_idt_entry(8, (uint64_t)&double_fault_handler, TRAP_GATE_32);
  //uint64_t *num = (uint64_t*)0xffffffffff;
  //*num = 50;
  kprintf("good!");
  __asm__("cli; hlt");
}
