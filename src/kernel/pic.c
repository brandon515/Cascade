#include "pic.h"

void PIC_sendEOI(uint8_t irq){
  if(irq >= 8){
    outb(PIC2_COMMAND,PIC_EOI);
  }
  outb(PIC1_COMMAND,PIC_EOI);
}

void io_wait(void){
  outb(0x80, 0); // empty port with a zero, just to make sure the previous operation has finished
}

void remap_PIC(int offset1, int offset2){
  __asm__("cli");
  uint8_t a1, a2;

  a1 = inb(PIC1_DATA); // get whatever masks is already on
  a2 = inb(PIC2_DATA);

  outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4); // init with the init fourth word
  io_wait();
  outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4); // init with the init fourth word
  io_wait();
  outb(PIC1_DATA, offset1); // tell the master it's offset
  io_wait();
  outb(PIC2_DATA, offset2); // tell the slave it's offset
  io_wait();
  outb(PIC1_DATA, 4); // tell the master that the slave is at IRQ2 (0000 0100)
  io_wait();
  outb(PIC2_DATA, 2);
  io_wait();

  outb(PIC1_DATA, ICW4_8086);
  io_wait();
  outb(PIC2_DATA, ICW4_8086);
  io_wait();

  outb(PIC1_DATA, a1); // now that it's init restore the former masks
  outb(PIC2_DATA, a2);
  __asm__("sti");
}


void IRQ_set_mask(uint8_t IRQline){
  uint16_t port;
  uint8_t value;

  if(IRQline < 8){ // anything below 8 belongs to the first PIC chip
    port = PIC1_DATA;
  }else{ // 9 and above is second PIC chip
    port = PIC2_DATA;
    IRQline -= 8;
  }
  value = inb(port) | (1 << IRQline); // set the bit for the PIC
  outb(port, value);
}

void IRQ_clear_mask(uint8_t IRQline){
  uint16_t port;
  uint8_t value;

  if(IRQline < 8){ // anything below 8 belongs to the first PIC chip
    port = PIC1_DATA;
  }else{ // 9 and above is second PIC chip
    port = PIC2_DATA;
    IRQline -= 8;
  }
  value = inb(port) & ~(1 << IRQline); // set the bit for the PIC
  outb(port, value);
}

static uint16_t __pic_get_irq_reg(int ocw3){
  outb(PIC1_COMMAND, ocw3);
  outb(PIC2_COMMAND, ocw3);
  return (inb(PIC1_COMMAND) << 8) | inb(PIC2_COMMAND);
}

uint16_t pic_get_irr(void){
  return __pic_get_irq_reg(PIC_READ_IRR);
}

uint16_t pic_get_isr(void){
  return __pic_get_irq_reg(PIC_READ_ISR);
}

void init_PIC(){
  remap_PIC(PIC_OFFSET, PIC_OFFSET+8);
}
