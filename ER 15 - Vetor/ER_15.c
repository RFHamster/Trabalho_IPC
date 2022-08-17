#include <stdio.h>

int main(void) {

  int vetorUm[10];
  int vetorDois[5];

  printf("Vetor 1 \n");
  for (int i = 0; i < 10; i++) {
    printf("V[%d]: \n", (i + 1));
    scanf("%d", &vetorUm[i]);
  }

  printf("Vetor 2 \n");
  for (int j = 0; j < 5; j++) {
    printf("V[%d]: \n", (j + 1));
    scanf("%d", &vetorDois[j]);
  }

  printf("== Vetor 1: ");
  for (int i = 0; i < 10; i++) {
    printf("%d ", vetorUm[i]);
  }

  printf("\n== Vetor 2: ");
  for (int i = 0; i < 5; i++) {
    printf("%d ", vetorDois[i]);
  }

  for (int i = 0; i < 10; i++) {
    printf("\nNumero %d\n", vetorUm[i]);
    int possuiDivisor = 0;
    for (int j = 0; j < 5; j++) {
      if (vetorUm[i] % vetorDois[j] == 0) {
        printf("Divisivel por %d na posicao %d\n", vetorDois[j], (j + 1));
        possuiDivisor = 1;
      }
    }
    if (possuiDivisor == 0) {
      printf("Nao possui Divisor\n");
    }
  }

  return 0;
}
