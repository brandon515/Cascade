#include "stdint.h"
//#include "printf.h"

typedef struct _memory_sector{
  uint32_t type;
  uint32_t size;
  uint32_t lower;
  uint32_t upper;
} memory_sector;

static volatile uint16_t *video = (uint16_t *) 0xb8000;
void kernel_main(uint32_t* info){
	return;
}
