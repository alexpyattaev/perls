//using 4 fields instead of 3 makes this code ~2x faster.
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[inline(never)]
pub fn vector_add_struct(left: Vector, right: Vector) -> Vector {
    return Vector {
        x: left.x + right.x,
        y: left.y + right.y,
        z: left.z + right.z,
        w: left.w + right.w,
    };
}

#[inline(never)]
pub fn vector_add_inplace(left:&mut Vector, right: Vector)  {

        left.x += right.x;
        left.y += right.y;
        left.z += right.z;
        left.w += right.w;
}

#[inline(never)]
pub fn vector_add_fields(
    left_x: f64,
    left_y: f64,
    left_z: f64,
    left_w: f64,
    right_x: f64,
    right_y: f64,
    right_z: f64,
    right_w: f64,
) -> Vector {
    return Vector {
        x: left_x + right_x,
        y: left_y + right_y,
        z: left_z + right_z,
        w: left_w + right_w,
    };
}

pub fn bench(mode: usize, length: usize) -> Vector {
    let mut result = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    match mode {
        0=>{
        for i in 0..length {
            let i = i as f64;
            result = vector_add_struct(
                result,
                Vector {
                    x: i,
                    y: i,
                    z: i,
                    w: i,
                },
            );
        }
    }
    1=>{
        for i in 0..length {
            let i = i as f64;
            result = vector_add_fields(result.x, result.y, result.z, result.w, i, i, i, i);
        }
    }
    2=>{
        for i in 0..length {
            let i = i as f64;
            vector_add_inplace(
                &mut result,
                Vector {
                    x: i,
                    y: i,
                    z: i,
                    w: i,
                },
            );
        }
    }
    _=>{}
    }
    result
}

fn main() {
    let mut arg = std::env::args().skip(1);
    let mode: usize = arg.next().unwrap().parse().unwrap();
    let length: usize = arg.next().unwrap().parse().unwrap();
    let result = bench(mode, length);
    if mode > 3{ // this never gets printed, its here to trick teh optimizer
    println!(
        "result { } {} {} {}",
        result.x, result.y, result.z, result.w
    );
    }
}
