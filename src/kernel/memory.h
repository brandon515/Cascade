#ifndef MEMORY_H_
#define MEMORY_H_
#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include "../arch/x86_64/start.h"
#include "printf.h"
#include "multiboot2.h"

typedef struct _Sector{
  size_t size;
  bool free;
  struct _Sector *next;
  struct _Sector *prev;
} Sector;

void *kalloc(size_t size);
void kfree(void* ptr);
struct multiboot_mmap_entry* init_memory(uint32_t* info);
void init_heap();

#endif
