#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void)
{
  FILE *pont_arq;
  char texto_str[50];
  char constante[2] = "M";
  int nenhum = 0;

  pont_arq = fopen("nomes.txt", "r");
  printf("-------------Iniciando Consulta-------------\n");
  printf("Produtos achados: \n\n");

  while(fgets(texto_str, 50, pont_arq) != NULL){
    int retorno = strncmp(texto_str,constante , 1);
    if(retorno == 0){
      printf("%s", texto_str);
      nenhum = 1;
    }
  }

  if(nenhum == 0){
    printf("Nenhum produto foi achado na consulta");
  }
  
  fclose(pont_arq);
  
  return(0);
}