#include<stdint.h>
#include<stdio.h>

extern int32_t add(int32_t one, int32_t two);

int main(){
    int one=5;
    int two=4;
    int output = add(one,two);
    printf("one= %d, two=%d",one,two);
}
