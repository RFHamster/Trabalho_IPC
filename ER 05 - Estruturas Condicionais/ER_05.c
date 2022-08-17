#include <stdio.h>
#include <stdlib.h>
#include <locale.h>

void main(){

    setlocale(LC_ALL, "");

    int n1, n2, n3, n4;

    printf("Digite três números em ordem crescente: ");
    scanf("%d", &n1);
    scanf("%d", &n2);
    scanf("%d", &n3);

    printf("\nDigite um número fora de ordem: ");
    scanf("%d", &n4);

    if(n4 > n3){
        printf("\nA ordem decrescente é: %d - %d - %d - %d", n4, n3, n2, n1);
    }else if( n4 > n2 && n4 < n3){
        printf("\nA ordem decrescente é: %d - %d - %d - %d", n3, n4, n2, n1);
    }else if(n4 > n1 && n4 < n2){
        printf("\nA ordem decrescente é: %d - %d - %d - %d", n3, n2, n4, n1);
    }else if(n4 < n1){
      printf("\nA ordem decrescente é: %d - %d - %d - %d", n3, n2, n1, n4);
    }
}
