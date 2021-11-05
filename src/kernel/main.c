#include <stdint.h>
#include "printf.h"
#include "gdt.h"
#include "../arch/x86_64/idt.h"
#include "interrupts.h"
#include "multiboot2.h"

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


void kmain(uint32_t* info, uint64_t* page_table, gdt_entry* gdt){
  cls();
  create_idt_entry(8, (uint64_t)&double_fault_handler, TRAP_GATE_32);
  //uint64_t *num = (uint64_t*)0xffffffffff;
  //*num = 50;
  info = (uint32_t*)((uint8_t*)info-2); // for some reason the process from real mode to 64-bit adds 2 bytes to the address
  uint32_t boot_info_size = info[0]/4; // Grub makes the first 4 bytes of the boot info the size of the structure in bytes
  for(uint32_t i = 0; i < boot_info_size; i++){
    if(info[i] == 6){
      multiboot_tag_mmap *mmap = (multiboot_tag_mmap*)&info[i];
      if(mmap->entry_size != 24){
        continue;
      }
      printf("Type: %d\n", mmap->type);
      printf("Size: %d\n", mmap->size);
      printf("Entry Size: %d\n", mmap->entry_size);
      printf("Entry Version: %d\n", mmap->entry_version);
      for(uint32_t j = 0; j < mmap->size; j++){
        if(mmap->entries[j].type > 5){
          break;
        }
        switch(mmap->entries[j].type){
          case MULTIBOOT_MEMORY_AVAILABLE:
            printf("Address: %x Length: %d Type: Available\n", mmap->entries[j].addr, mmap->entries[j].len);
            break;
          case MULTIBOOT_MEMORY_RESERVED:
            printf("Address: %x Length: %d Type: Reserved\n", mmap->entries[j].addr, mmap->entries[j].len);
            break;
          case MULTIBOOT_MEMORY_ACPI_RECLAIMABLE:
            printf("Address: %x Length: %d Type: ACPI Reclaimable\n", mmap->entries[j].addr, mmap->entries[j].len);
            break;
          case MULTIBOOT_MEMORY_NVS:
            printf("Address: %x Length: %d Type: NVS\n", mmap->entries[j].addr, mmap->entries[j].len);
            break;
          case MULTIBOOT_MEMORY_BADRAM:
            printf("Address: %x Length: %d Type: Bad Ram\n", mmap->entries[j].addr, mmap->entries[j].len);
            break;
          default:
            break;
        }
      }
    }
  }
  printf("good!");
  __asm__("cli; hlt");
}
