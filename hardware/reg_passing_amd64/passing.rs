pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[inline(never)]
pub fn vector_add_struct(left: Vector, right: Vector) -> Vector {
    return Vector {
        x: left.x + right.x,
        y: left.y + right.y,
        z: left.z + right.z,
    };
}
#[inline(never)]
pub fn vector_add_fields(
    left_x: f64,
    left_y: f64,
    left_z: f64,
    right_x: f64,
    right_y: f64,
    right_z: f64,
) -> Vector {
    return Vector {
        x: left_x + right_x,
        y: left_y + right_y,
        z: left_z + right_z,
    };
}

pub fn bench(mode: usize, length: usize) -> Vector {
    let mut result = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    if mode == 0 {
        for i in 0..length {
            let i = i as f64;
            result = vector_add_struct(result, Vector { x: i, y: i, z: i });
        }
    } else {
        for i in 0..length {
            let i = i as f64;
            result = vector_add_fields(result.x, result.y, result.z, i, i, i);
        }
    }
    result
}

fn main() {
    let mut arg = std::env::args().skip(1);
    let mode: usize = arg.next().unwrap().parse().unwrap();
    let length: usize = arg.next().unwrap().parse().unwrap();
    let result = bench(mode, length);
    println!("result { } {} {}", result.x, result.y, result.z);
}
