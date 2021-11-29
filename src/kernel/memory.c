#include "memory.h"


void *kalloc(size_t size){
  // get the first in the linked list
  Sector *prev = get_heap_start();
  Sector *mem_sect = prev->next;
  // see if there's a entry in the linked list that is as big or bigger than the requested size
  while(mem_sect != NULL){
    if(mem_sect->size >= size && mem_sect->free){
      break;
    }
    prev = mem_sect;
    mem_sect = mem_sect->next;
  }
  // if mem_sect is null then there is no block with the requested size
  if(mem_sect == NULL){
    return NULL;
  }

  // preserve the mem_sect entry in the linked list
  void *ret = ((uint8_t*)mem_sect+sizeof(Sector));

  // create the new memory segment
  Sector *new_mem = (Sector*)((uint8_t*)ret+(sizeof(Sector)+size));
  new_mem->size = mem_sect->size-(sizeof(Sector)+size);
  new_mem->free = true;

  //insert the new memory into the linked list
  new_mem->next = mem_sect->next;
  new_mem->prev = mem_sect;

  if(mem_sect->next != NULL){
    mem_sect->next->prev = new_mem;
  }

  mem_sect->next = new_mem;

  // resize the now used sector and mark it as used
  mem_sect->size = sizeof(Sector)+size;
  mem_sect->free = false;

  return ret;
}

void kfree(void *ptr){
  Sector *mem_sect = (Sector*)((uint8_t*)ptr-sizeof(Sector));
  mem_sect->free = true;
  
  // check if the next sector is free, if so then combine the two
  if(mem_sect->next != NULL && mem_sect->next->free == true){
    mem_sect->size = mem_sect->size+mem_sect->next->size;
    mem_sect->next = mem_sect->next->next;
  }
  // the reason we do the next first so that if the previous is also free then the
  // algorithm doesn't have to change. mem_sect->next has already become mem_sect->next->next

  // check if the previous sector is free, if so then combine them
  if(mem_sect->prev->free == true){
    mem_sect->prev->size = mem_sect->prev->size+mem_sect->size;
    mem_sect->prev->next = mem_sect->next;
  }
}

struct multiboot_mmap_entry* init_memory(uint32_t* info){
  printf("INITILIZING MEMORY MAP\n");
  uint32_t boot_info_size = info[0]/4; // Grub makes the first 4 bytes of the boot info the size of the structure in bytes
  int count = 0;
  struct multiboot_mmap_entry* stack_map;
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
            stack_map[count].addr = mmap->entries[j].addr;
            stack_map[count].len = mmap->entries[j].len;
            stack_map[count].type = mmap->entries[j].type;
            stack_map[count].zero = mmap->entries[j].zero;
            count++;
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
  struct multiboot_mmap_entry* heap_map = kalloc(sizeof(struct multiboot_mmap_entry)*count);
  for(int i = 0; i < count; i++){
    if(stack_map[i].addr != 0){ // let's not use the lower memory regions
      heap_map[i].addr= stack_map[i].addr;
      heap_map[i].len = stack_map[i].len;
      heap_map[i].type= stack_map[i].type;
      heap_map[i].zero= stack_map[i].zero;
      printf("Address: %x Length: %d Type: Available\n", heap_map[i].addr, heap_map[i].len);
    }
  }
  printf("MEMORY MAP INITILIZED\n");
  return heap_map;
}

void init_heap(){
  printf("INITILIZING KERNEL HEAP\n");
  Sector *head = (Sector*)get_heap_start();
  head->size = -1;
  head->next = (head+1);
  head->prev = NULL;
  head->free = false;
  Sector *first = head->next;
  first->size = 0x8000-sizeof(Sector); // the size of the heap without the head
  first->next = NULL;
  first->prev = head;
  first->free = true;
  printf("KERNEL HEAP INITILIZED\n");
}
