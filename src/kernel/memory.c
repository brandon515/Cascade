#include "memory.h"


void *kalloc(size_t size){
  Sector *prev = get_heap_start();
  Sector *mem_sect = prev->next;
  while(mem_sect != NULL){
    if(mem_sect->size >= size){
      break;
    }
    prev = mem_sect;
    mem_sect = mem_sect->next;
  }
  if(mem_sect == NULL){
    return NULL;
  }

  void *ret = (void*)mem_sect;

  if(mem_sect->size == size){
    prev->next = mem_sect->next;
  }else{
    prev->next = (Sector*)((uint8_t*)mem_sect+size);
    prev->next->size = mem_sect->size-size;
    prev->next->next = mem_sect->next;
  }
  return ret;
}

void free(void *ptr){
  Sector *new_mem = (Sector*)ptr;
  Sector *head = get_heap_start();
  while(head != NULL){
    if(head->next > new_mem){
      break;
    }
  }
}
