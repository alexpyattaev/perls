use std::ops::Shl;

struct Rsout;
struct Endl;

impl<'a> Shl<&'a str> for Rsout {
    type Output = Rsout;

    fn shl(self, _rhs: &'a str) -> Rsout {
        print!("{}", _rhs);
        self
    }
}

impl Shl<i32> for Rsout {
    type Output = Rsout;

    fn shl(self, _rhs: i32) -> Rsout {
        print!("{}", _rhs);
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
    Rsout << "Hello, " << "world!" << Endl;
    Rsout << "1 + 2 = " << 1 + 2 << Endl;
}
