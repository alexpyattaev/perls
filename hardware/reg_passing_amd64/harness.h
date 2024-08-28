#define TYPE double

struct Vector { TYPE x, y, z; };

struct Vector vector_add_struct(struct Vector left, struct Vector right);

struct Vector vector_add_fields(
    TYPE left_x, TYPE left_y, TYPE left_z,
    TYPE right_x, TYPE right_y, TYPE right_z);


