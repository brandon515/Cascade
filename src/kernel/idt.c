#include "../arch/x86_64/idt.h"

/*
typedef struct IDTDescr {
   uint16_t offset_1; // offset bits 0..15
   uint16_t selector; // a code segment selector in GDT or LDT
   uint8_t ist;       // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
   uint8_t type_attr; // type and attributes
   uint16_t offset_2; // offset bits 16..31
   uint32_t offset_3; // offset bits 32..63
   uint32_t zero;     // reserved
} idt_entry;
*/

static idt_entry idt[IDT_ENTRIES];

void create_idt_entry(int idt_index, uint64_t function, uint8_t flags){
  //idt_entry *entry = &idt[idt_index];
  idt[idt_index].offset_1 = function&0xffff;
  idt[idt_index].offset_2 = (function>>16)&0xff;
  idt[idt_index].offset_3 = function>>32;
  idt[idt_index].ist = 0; // this is set to 0 in 64 bit mode
  idt[idt_index].selector = 8; // GDT Code Sector, this is the offest in bytes
  idt[idt_index].type_attr = flags;//0x8f;
  idt[idt_index].zero = 0;
}

void load_idt_entries(){
  create_idt_entry(8, (uint64_t)&double_fault_handler, TRAP_GATE_64); // create double fault handler so we don't go into a triple fault reset cycle
  create_idt_entry(PIC_OFFSET, (uint64_t)&timer_handler, INT_GATE_64); // timer handler
}

void init_idt(){
  printf("INITILIZING IDT\n");
  idt_descriptor idt_des;
  idt_des.limit = sizeof(idt_entry)*IDT_ENTRIES;
  idt_des.start = &idt[0];
  load_idt(&idt_des);
  load_idt_entries();
  printf("IDT INITILIZED\n");
}

