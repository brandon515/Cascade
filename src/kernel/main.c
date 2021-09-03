#define IDT_ENTRIES 256
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

idt_entry idt[IDT_ENTRIES];

void kmain(memory_sector* info, uint64_t* page_table, gdt_entry* gdt){
  cls();
  idt_descriptor idt_des;
  idt_des.limit = sizeof(idt_entry)*IDT_ENTRIES;
  idt_des.start = &idt[0];
  load_idt(&idt_des);
  idt[8] = create_idt_entry((uint64_t)double_fault_handler);
  kprintf("0x%x", gdt);
  //uint64_t* blah = ((uint64_t*)0xffffffffff);
  //*blah = 50;
  return;
}
