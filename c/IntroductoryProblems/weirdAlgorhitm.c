#include <stdio.h>


int main(){
    int n;

    scanf("%d",&n);
    printf("*%d\n",n);
    printf("*%d\n",*n);
    printf("*%d\n",&n);
}