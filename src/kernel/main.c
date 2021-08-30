#include "stdint.h"
#include "printf.h"
#include "multiboot.h"


void kernel_main(void* info, uint32_t magic){
  cls();
  kprintf("type: %d", boot_info[0].type);
  kprintf("size: %d", boot_info[0].size);
  kprintf("lower: %d", boot_info[0].lower);
  kprintf("upper: %d", boot_info[0].upper);
	return;
}
