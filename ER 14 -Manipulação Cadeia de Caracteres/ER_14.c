#include <stdio.h>
#include <string.h>
#include <ctype.h>

int main() {

    char frase[100];
    int i;

    printf("Digite uma frase: ");
    scanf("%[^\n]", frase);
    
    for (i = 0; i < strlen(frase) + 1; i++) {
        if (i == 0 || frase[i - 1] == ' ')
            frase[i] = toupper(frase[i]);
        else
            frase[i] = frase[i];
    }

    printf("Frase: %s\n", frase);

    return 0;
}
