#include <stdint.h>
#include "printf.h"
#include "gdt.h"
#include "../arch/x86_64/idt.h"
#include "interrupts.h"
#include "multiboot2.h"
#include "../arch/x86_64/start.h"
#include "memory.h"
#include <stdbool.h>
#include "pic.h"

/*struct multiboot_mmap_entry
{
  multiboot_uint64_t addr;
  multiboot_uint64_t len;
#define MULTIBOOT_MEMORY_AVAILABLE              1
#define MULTIBOOT_MEMORY_RESERVED               2
#define MULTIBOOT_MEMORY_ACPI_RECLAIMABLE       3
#define MULTIBOOT_MEMORY_NVS                    4
#define MULTIBOOT_MEMORY_BADRAM                 5
  multiboot_uint32_t type;
  multiboot_uint32_t zero;
};*/


void kmain(uint32_t* info, gdt_entry* gdt){
  cls();
  printf("INITILIZING KERNEL HEAP\n");
  init_heap();
  printf("KERNEL HEAP INITILIZED\n");
  printf("INITILIZING AND REMAPPING PIC\n");
  init_PIC();
  printf("PIC PREPARED\n");
  printf("INITILIZING IDT\n");
  init_idt();
  printf("IDT INITILIZED\n");
  printf("INITILIZING MEMORY MAP\n");
  info = (uint32_t*)((uint8_t*)info-2); // for some reason the transition from real mode to 64-bit adds 2 bytes to the address
  struct multiboot_mmap_entry* a_mem = init_memory(info);
  printf("MEMORY MAP INITILIZED\n");
  while(true){}
}
