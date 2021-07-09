#include <stdio.h>
#include <stdlib.h>

#define MAX 300000

int main(){
    int n;
    char *s = malloc(MAX);
    if(s == NULL){
        printf("No Memory left\n");
        return 1;
    }
    // scanf("%d",&n);
    fgets(s,MAX,stdin);
    printf("%s",s);
}