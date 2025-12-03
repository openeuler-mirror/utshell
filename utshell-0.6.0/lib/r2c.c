#include <stdio.h>

#include <string.h>

 

int add(int i1, int i2) {

    return i1+i2;

}

 

void replace(__int8_t s[3], __int8_t d[3]) {

    int i = 0;

    for (i = 0; i < 3 ; i++) {

        s[i] += 33;

    }

    memset(d, 0xFF, sizeof(*d)*3);

    return;

}

 

void add_v2(int i1, int i2, int *ouput) {

    *ouput = i1 + i2;

    return;
}