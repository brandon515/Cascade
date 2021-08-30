#define COLUMNS				80
#define LINES					25
#define ATTRIBUTE			(7<<8)
#define VIDEO					0xB8000
#include <stdarg.h>
#include "stdint.h"

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
}


void cls(void){
	for(int i = 0; i < COLUMNS*LINES; i++){
		video[i] = 0;
	}
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
          char buf[16];
          int b = 15;
          while(b >= 0){
            uint8_t num = arg%16;
            arg = arg/16;
            if(num < 10){
              buf[b] = num+48;
            }else{
              switch(num){
                case 10:
                  buf[b] = 'A';
                  break;
                case 11:
                  buf[b] = 'B';
                  break;
                case 12:
                  buf[b] = 'C';
                  break;
                case 13:
                  buf[b] = 'D';
                  break;
                case 14:
                  buf[b] = 'E';
                  break;
                case 15:
                  buf[b] = 'F';
                  break;
              }
            }
            b--;
          }
          for(int z = 0; z < 16; z++){
            kputchar(buf[z]);
          }
          i++;
          break; //case 'x'
        case 'd':
          char* buf2;
          int buf_len = 0;
          int b2 = 0;
          while(b2 > 0){
            uint8_t num = arg%10;
            arg = arg/10;
            buf2[b2] = num+48;
            buf_len++;
            b2++;
          }
          for(int k = 0; k < buf_len; k++){
            kputchar(buf2[k]);
          }
          i++;
          break;
      }
		}else{
      kputchar(format[i]);
      i++;
    }
	}
}
