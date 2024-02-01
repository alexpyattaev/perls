#include <stdio.h>
//const_cast
//static_cast

int boo(const char* x)
{
    return printf("BBB %s", x);
}

int foo(const char* x)
{
    return printf("FFF %s", x);
}

int main(void)
{
     auto x = printf;
     x("boo\n");
     x = foo;
     x("fuu\n");
     /*
     printf("boo\n");
     {
        auto printf = foo;
        printf("fuu\n");
     }
     printf("boo\n");
     {
         auto printf = boo;
        printf("fuu\n");
     }*/
}
