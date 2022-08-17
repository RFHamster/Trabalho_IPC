#include <stdio.h>

int main(void) {
  
  int idade;
  float altura;
  int cont = 0;
  float mediaAltura = 0;

  while(1){
    printf("Digite a idade e a altura: \n");
    scanf("%d", &idade);

    if(idade <= 0){
      break;
    }
    
    scanf("%f", &altura);
  
    if(idade > 50){
      mediaAltura += altura;
      cont++;
    }
    
  }

  printf("A media da altura eh: %.2f", mediaAltura/cont);
  
  return 0;
}
