#ifndef GDT_H_
#define GDT_H_
#include <stdint.h>
typedef struct _gdt_entry{
  uint16_t lower_limit;
  uint16_t lower_base;
  uint8_t mid_base;
  uint8_t access;
  uint8_t upper_limit_flags;
  uint8_t upper_base;
}__attribute__((packed)) gdt_entry;
void create_gdt_entry(gdt_entry* entry, uint32_t base, uint32_t limit, uint8_t access, uint8_t flags);
#endif
