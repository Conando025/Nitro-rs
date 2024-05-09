#include "system.h"
#include <stdio.h>
#include <stdlib.h>

int main() {

    PERSONAL_DATA pd = {
        .RESERVED0 = {0x05, 0x00},
        .theme = 0x4,
        .birthMonth = 3,
        .birthDay = 8,
        .RESERVED1 = 0xFF,
        .name = {0x42, 0x65, 0x6e, 0x69},
        .nameLen = 4,
        .message = {0x04e,0x069,0x074,0x072,0x06f,0x02d,0x072,0x073,},
        .messageLen = 8,
        .alarmHour = 13,
        .alarmMinute = 24,
        .RESERVED2 = {0x02, 0xFF, 0xFC, 0xD4},
        .calX1 = 0x1234,
        .calY1 = 0x8765,
        .calX1px = 0xAB,
        .calY1px = 0xDC,
        .calX2 = 0x2345,
        .calY2 = 0x9876,
        .calX2px = 0xBC,
        .calY2px = 0xED,
        
        .language = 7,
        .gbaScreen = 0,
        .defaultBrightness = 0,
        .autoMode = 0,
        .RESERVED5 = 0,
        .settingsLost = 0,
        .RESERVED6 = 0x3f,
        
        .RESERVED3 = 0x0123,
        .rtcOffset = 1,
        .RESERVED4 = 0x12345678,
    };
    printf("The struct is %ld bytes in C.\n", sizeof(pd));

    // file pointer variable to store the value returned by
    // fopen
    FILE* fptr;

    // opening the file in read mode
    fptr = fopen("example.obj", "wb");
 
    // checking if the file is opened successfully
    if (fptr == NULL) {
        printf("The file is not opened. The program will now exit.\n");
        exit(0);
    }

    fwrite(&pd, sizeof(pd), 1, fptr);
    fclose(fptr);
 
    return 0;
}   
