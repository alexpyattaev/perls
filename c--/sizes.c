#include <stdio.h>

int main(void){
	printf("short int %ld\n", sizeof(short int));
	printf("int %ld\n", sizeof(int));
	printf("long int %ld\n", sizeof(long int));
	printf("long long int %ld\n", sizeof(long long int));

	printf("%ld\n", sizeof(float));
	printf("%ld\n", sizeof(double));
	printf("%ld\n", sizeof(long double));
}
