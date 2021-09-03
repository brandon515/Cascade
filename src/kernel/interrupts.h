#ifndef INTERRUPTS_H_
#define INTERRUPTS_H_
#include "stdint.h"

void double_fault_handler(void* interrupt_stack, uint64_t error_code);
#endif
