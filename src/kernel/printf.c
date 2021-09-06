#define COLUMNS				80
#define LINES					25
#define ATTRIBUTE			(7<<8)
#define VIDEO					0xB8000
#include <stdarg.h>
#include <stdint.h>

static volatile uint16_t *video = (uint16_t *) VIDEO;
static int xpos = 0;

static void newline(void){
  xpos = 0;
  for(int y = 1; y < LINES+1; y++){
    for(int x = 0; x < COLUMNS+1; x++){
      video[(((y-1)*COLUMNS)+x)] = video[((y*COLUMNS)+x)];
    }
  }
}

static void kputchar (char c) {
  if (c == '\n' || c == '\r'){
    newline();
    return;
  }

  video[((LINES-1)*COLUMNS)+xpos] = c+ATTRIBUTE;
  //*(video + (xpos + (LINES-1) * COLUMNS) * 2 + 1) = ATTRIBUTE;

  xpos++;
  if (xpos >= COLUMNS){
    newline();
  }
  return;
}


void cls(void){
	for(uint16_t i = 0; i < COLUMNS*LINES; i++){
		video[i] = 0;
	}
  return;
}

void kprintf (char *format, ...) {
  va_list others;
  va_start(others, format);
	int i = 0; // character position in format
	while(format[i] != 0){
		if(format[i] == '%'){
      i++; // increment the position in the format to see what the next letter is
      uint64_t arg = va_arg(others, uint64_t);
      switch(format[i]){
        case 'x': // hexadecimal
          if(arg == 0) {
            kputchar('0'); // output 0
            i++; // don't print the x in %x
            break;
          }
          char buf[16] = {0}; // initialize the array to all 0's, it's 16 because we're in 64-bit
          int b = 0;
          while(arg != 0){ //while there are still numbers to process in the argument
            uint8_t num = arg%16; // get the lowest hex number in the arguement so F in 0x342F
            arg = arg/16; // pop the lowest number of the hex number so now the number looks liks 0x0342
            if(num < 10){ // if the number is less than 10 then it's just a normal number
              buf[b] = num+48; // 48 is ascii for 0 so 0+48='0' and 1+48='1'
            }else{
              buf[b] = num+55; // 65 is ascii for A and we can garuntee it's above 10 so 10+55='A' and 11+55='B'
            }
            b++; // increase the char array pointer
          }
          for(int z = b-1; z >= 0; z--){ // the array is backwards so we start at the end and work our way back
            kputchar(buf[z]);
          }
          i++; // don't print the x in %x
          break; //case 'x'
        case 'd':
          if(arg == 0){
            kputchar('0');
          }
          char* buf2 = {0};
          int b2 = 0;
          while(arg != 0){
            char num = arg%10;
            arg = arg/10;
            buf2[b2] = num+48; // 48 is ascii for '0', see above
            b2++;
          }
          for(int k = b2-1; k >= 0; k--){
            kputchar(buf2[k]);
          }
          i++; // don't print the d in %d
          break; // case 'd'
      }
		}else{
      kputchar(format[i]);
      i++;
    }
	}
}
