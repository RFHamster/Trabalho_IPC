#include <stdio.h>

void main(){

    double PI = 3.14159;
    double raio;

    printf("Digite o raio: \n");
    scanf("%lf", &raio);

    printf("Comprimento = %.2lf\n", (2 * PI * raio));
	  printf("Area = %.2lf\n", (PI * raio * raio));
    printf("Volume = %.2lf", ((3 * PI * raio * raio * raio)/4));

}
