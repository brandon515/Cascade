#ifndef INTERRUPTS_H_
#define INTERRUPTS_H_
#include <stdint.h>
#include <stdbool.h>
#include "printf.h"
#include "pic.h"
#include "../arch/x86_64/io.h"
#include "keyboard.h"

static uint64_t timer_count = 0;
void double_fault_handler(void* interrupt_stack, uint64_t error_code);
void timer_handler(void* interrupt_stack);
void keyboard_handler(void* interrupt_stack);
#endif
