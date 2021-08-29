#include "stdint.h"
#include "printf.h"
#include "multiboot.h"

void kernel_main(unsigned long magic){
  cls();
	kprintf("tester testing test 0x%x", 0x5353abcdef);
	return;
}
