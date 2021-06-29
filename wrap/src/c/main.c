#include <stdio.h>
#include "wrap.h"

int main(void){
    double ret = mytan(1.0);
    printf("mytan(1.0) = %f\n", ret);
    return 0;
}