#include<stdio.h>
float bad_add(float v1, float v2){
    printf("C: The actual result should be: %f", v1+v2);
    printf("\nC: This badmath library returns: %f\n", v1+v2+10.0);
    return v1+v2+10.0;
}