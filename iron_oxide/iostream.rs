use std::ops::Shl;

struct Rsout;
struct Endl;

impl <T> Shl<T> for Rsout
where T: std::fmt::Display
{
    type Output = Rsout;

    fn shl(self, rhs: T) -> Rsout {
        print!("{}", rhs);
        self
    }
}

impl Shl<Endl> for Rsout {
    type Output = Rsout;

    fn shl(self, _rhs: Endl) -> Rsout {
        print!("\n");
        self
    }
}


fn main() {
    let x = 5;
    // println!("{&x}"); //Does not compile!
    Rsout<< &x << Endl;
    Rsout << "Hello, " << "world!" << Endl;
    Rsout << "1 + 2 = " << 1 + 2 << Endl;
}
