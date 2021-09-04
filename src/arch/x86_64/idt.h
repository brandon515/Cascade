#ifndef IDT_H_
#define IDT_H_
#include <stdint.h>
typedef struct IDTDescr {
   uint16_t offset_1; // offset bits 0..15
   uint16_t selector; // a code segment selector in GDT or LDT
   uint8_t ist;       // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
   uint8_t type_attr; // type and attributes
   uint16_t offset_2; // offset bits 16..31
   uint32_t offset_3; // offset bits 32..63
   uint32_t zero;     // reserved
}__attribute__((packed)) idt_entry;
typedef struct _idt_descriptor {
  uint16_t limit;
  idt_entry* start;
}__attribute__((packed)) idt_descriptor;
void load_idt(idt_descriptor*);
void create_idt_entry(idt_entry* entry, uint64_t function);
#endif

