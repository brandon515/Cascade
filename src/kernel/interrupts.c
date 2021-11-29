#include "interrupts.h"

__attribute__((interrupt))
void double_fault_handler(void* interrupt_stack, uint64_t error_code){
  printf("Double Fault");
  __asm__("hlt");
}

__attribute__((interrupt))
void timer_handler(void* interrupt_stack){
  printf("timer");
  PIC_sendEOI(0);
}
