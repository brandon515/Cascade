#include <stdint.h>
#include "../arch/x86_64/boot.h"
#include "gdt.h"
#include "printf.h"
#include "multiboot2.h"

void bootstrap(uint32_t info, uint32_t magic, uint32_t gdt){
  uint32_t pass_info = info;
  if(magic != MULTIBOOT2_BOOTLOADER_MAGIC){
    error('0');
  }
  check_cpuid();
  check_long_mode();
  uint32_t page_table = enable_paging();
  enable_long_mode(pass_info, gdt); // this is an assembly function that does an unconditional jump to long_mode_start in start.asm
	return; // this is never reached
}
