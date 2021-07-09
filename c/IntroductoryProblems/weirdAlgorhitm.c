#include <stdio.h>


int main(){
    int n;
    scanf("%d",&n);


    while(n!=1){
        printf("%d ",n);
        if(n%2)n = n*2+3;
        else n << 1;
    }


    printf("\n");
}