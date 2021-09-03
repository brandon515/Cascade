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

void create_idt_entry(idt_entry* entry, uint64_t function){
  entry->offset_1 = function&0xffff;
  entry->offset_2 = (function>>16)&0xff;
  entry->offset_3 = function>>32;
  entry->ist = 0; // this is set to 0 in 64 bit mode
  entry->selector = 8; // GDT Code Sector, the kernel code is 1
  entry->type_attr = 0x8f;
  entry->zero = 0;
}
