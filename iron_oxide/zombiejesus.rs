fn dots() {
    assert_eq!(String::from(".................................................."),
               format!("{:?}", .. .. .. .. .. .. .. .. .. .. .. .. ..
                               .. .. .. .. .. .. .. .. .. .. .. ..));
}

fn punch_card() -> impl std::fmt::Debug {
    ..=..=.. ..    .. .. .. ..    .. .. .. ..    .. ..=.. ..
    ..=.. ..=..    .. .. .. ..    .. .. .. ..    ..=..=..=..
    ..=.. ..=..    ..=.. ..=..    .. ..=..=..    .. ..=.. ..
    ..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..
    ..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=.. ..
}

#[derive(Debug)]
struct A{
x:u32,
y:u32
}

impl std::ops::Index<u32> for A{
     type Output = u32;

    fn index(&self, index: u32) -> &Self::Output{
        todo!()
    }
}

impl std::ops::Index<(u32,u32)> for A{
     type Output = &'static [u32];

    fn index(&self, index: (u32,u32)) -> &Self::Output{
        todo!()
    }
}
impl std::ops::Index<(u32,std::ops::Range<u32>)> for A{
     type Output = &'static [u32];

    fn index(&self, index: (u32,std::ops::Range<u32>)) -> &Self::Output{
        todo!()
    }
}

fn main(){
    dbg!(punch_card());
    let x = [1,2,3,4];
    let b = A{x:3, y:0};
    let a = A{y:42, ..b};
    let s = std::ops::RangeFull{};
    a[42];
    a[(3,2)];
    a[(3,2..5)];
    dbg!(s);
    dbg!(&x[s]);
    dots()
}
