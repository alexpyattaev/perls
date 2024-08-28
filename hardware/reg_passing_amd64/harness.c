
#include <stdio.h>
#include <stdlib.h>
#include "harness.h"

int main(int argc, const char *argv[])
{
    int mode = atoi(argv[1]);
    int length = atoi(argv[2]);
    struct Vector result = {0};
    if (mode == 0)
    {
        for (int i = 0; i < length; i++)
            result = vector_add_struct(result, (struct Vector) {i, i, i});
    }
    else
    {
        for (int i = 0; i < length; i++)
            result = vector_add_fields(result.x, result.y, result.z, i, i, i);
    }
    printf("result <%f, %f, %f>\n", result.x, result.y, result.z);
}

