#include <stdio.h>
#include <stdlib.h>

void main(){

		int m[10][10];
		int l, c, media;
		media = 0;

		for(l = 0; l < 10; l++){
			for(c = 0; c < 10; c++){
        printf("Digite M[%d][%d]: \n", l, c);
        scanf("%d", &m[l][c]);
			}
		}
		for(l = 0; l < 10; l++){
			for(c = 0; c < 10; c++){
				if(l == c){
					media += m[l][c];
				}
			}
		}
		printf("Media dos numeros na diagonal: %d", media/10);
}
