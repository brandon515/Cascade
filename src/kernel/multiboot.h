#ifndef MULTIBOOT_H
#define MULTIBOOT_H
#include "stdint.h"

typedef struct _memory_sector{
  uint32_t type;
  uint32_t size;
  uint32_t lower;
  uint32_t upper;
} memory_sector;

static memory_sector* boot_info;

void save_info(memory_sector*);
#endif
