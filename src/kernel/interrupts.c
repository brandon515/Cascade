#include "interrupts.h"
#include "printf.h"

__attribute__((interrupt))
void double_fault_handler(void* interrupt_stack, uint64_t error_code){
  kprintf("Double Fault");
}
