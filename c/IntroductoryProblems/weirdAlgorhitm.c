#include <stdio.h>


int main(){
    int n;
    scanf("%d",&n);

    while(n!=1){
        printf("%d ",n);
        if(n%2){
            n = n*3+1;
        }else {
            n = n/2;
        }
    }


    printf("%d\n",n);
}