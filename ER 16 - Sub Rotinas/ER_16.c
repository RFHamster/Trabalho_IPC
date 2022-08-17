#include <stdio.h>

void arrumaVetor(int *A, int *B, int *C);

int main()
{
    int A[10];
    int B[10];
    int C[20];

    for(int i = 0; i < 10; i++){
        printf("Digite o A[%d]\n", i + 1);
        scanf("%d", &A[i]);
    }

    for(int i = 0; i < 10; i++){
        printf("Digite o B[%d]\n", i + 1);
        scanf("%d", &B[i]);
    }

    arrumaVetor(A, B, C);
    
    printf("\nVetor C: ");
    
    for(int i = 0; i < 20; i++){
        printf("%d ", C[i]);
    }

    return 0;
}

void arrumaVetor(int *A, int *B, int *C){
    int maior;

    for(int i = 0; i < 10; i++){
       C[i] = A[i]; 
    }

    for(int i = 0; i < 10; i++){
       C[i+10] = B[i]; 
    }


    for(int i = 0; i < 20; i++){
        for(int j = i+1; j < 20; j++){
            if(C[i] < C[j]){
                maior = C[j];
                C[j] = C[i];
                C[i] = maior;
            }
        }
    }
  }
