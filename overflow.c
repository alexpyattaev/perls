#include <stdio.h>
#include <stdint.h>

void foo(int32_t a) {
  int32_t X = a+100;
  if (X > a)
  {
      printf("No overflow\n");
  }
  else
  {
      printf("Overflow occured\n");
  }
  if (X > 0)
  {
      printf("No overflow\n");
  }
  else
  {
      printf("Overflow occured\n");
  }
  printf("%d %d\n",X, a);

}

int main() {
  printf(">>> TEST 1 - Should not overflow\n");
  foo(100);
  printf(">>> TEST 2 - Should overflow\n");
  foo(0x7fffffff);
}
