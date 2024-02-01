#include <stdio.h>

struct E{};

struct E1{
char t;
};
struct E2{
double t;
};

struct A{
	char t;
	char t2;
double x;
	char t3;
double y;
	char t4;
	char t5;
};


int main (void){
   struct A killme;
   printf("%lu\n", sizeof(double));
   printf("%lu\n", sizeof(struct E));
   printf("%lu\n", sizeof(struct E1));
   printf("%lu\n", sizeof(struct E2));
   printf("%lu\n", sizeof(killme));
}

