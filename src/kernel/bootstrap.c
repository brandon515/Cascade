#include <stdint.h>
#include "../arch/x86_64/boot.h"
#include "gdt.h"
#include "printf.h"

void bootstrap(uint32_t info, uint32_t magic, uint32_t gdt){
  if(magic != 0x36d76289){
    error('0');
  }
  check_cpuid();
  check_long_mode();
  uint32_t page_table = enable_paging();
  enable_long_mode(info, page_table, gdt);
  //load_gdt();
  //jump_64_bit(info, page_table, gdt);
	return;
}
