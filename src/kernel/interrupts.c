#include "interrupts.h"

__attribute__((interrupt))
void double_fault_handler(void* interrupt_stack, uint64_t error_code){
  printf("Double Fault");
  __asm__("hlt");
}

__attribute__((interrupt))
void timer_handler(void* interrupt_stack){
  static uint64_t timer_count;
  timer_count++;
  PIC_sendEOI(0);
}

__attribute__((interrupt))
void keyboard_handler(void* interrupt_stack){
  static bool left_shift_pressed, right_shift_pressed;
  uint8_t ret = inb(0x60);
  if(ret == 0x2A){
    left_shift_pressed = true;
  }else if(ret == 0xAA){
    left_shift_pressed = false;
  }
  if(ret == 0x36){
    right_shift_pressed = true;
  }else if(ret == 0xB6){
    right_shift_pressed = false;
  }
  uint8_t key = kbdus[ret];
  if((left_shift_pressed || right_shift_pressed) && (key >= 97 && key <= 122)){
    key -= 32;
  }
  if(ret <= 127){
    printf("%c", key);
  }
  PIC_sendEOI(1);
}
