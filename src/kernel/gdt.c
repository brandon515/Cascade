#include "gdt.h"
/*
typedef struct _gdt_entry{
  uint16_t lower_limit;
  uint16_t lower_base;
  uint8_t mid_base;
  uint8_t access;
  uint8_t upper_limit_flags;
  uint8_t upper_base;
} gdt_entry;
*/

void create_gdt_entry(gdt_entry* entry, uint32_t base, uint32_t limit, uint8_t access, uint8_t flags){
  entry->lower_limit = limit & 0xFFFF;
  entry->upper_limit_flags = limit  >> 16;
  entry->upper_limit_flags += flags << 4;
  entry->lower_base = base & 0xFFFF;
  entry->mid_base = base >> 16;
  entry->upper_base = base >> 24;
  entry->access = access;
}
