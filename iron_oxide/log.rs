fn main() {
    let x = 9f64.log(3.);
    println!("Hello, world! {}", x );
    println!("{}", f64::EPSILON );

    assert_eq!(x , 2.)
}
