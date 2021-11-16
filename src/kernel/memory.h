#ifndef MEMORY_H_
#define MEMORY_H_
#include <stddef.h>
#include <stdint.h>
#include "../arch/x86_64/start.h"

typedef struct _Sector{
  size_t size;
  struct _Sector *next;
} Sector;

void *kalloc(size_t size);

#endif
