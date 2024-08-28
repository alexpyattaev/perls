
#include "harness.h"

struct Vector vector_add_struct(struct Vector left, struct Vector right)
{
    return (struct Vector) {
        left.x + right.x,
        left.y + right.y,
        left.z + right.z,
    };
}

struct Vector vector_add_fields(
    TYPE left_x, TYPE left_y, TYPE left_z,
    TYPE right_x, TYPE right_y, TYPE right_z)
{
    return (struct Vector) {
        left_x + right_x,
        left_y + right_y,
        left_z + right_z,
    };
}

