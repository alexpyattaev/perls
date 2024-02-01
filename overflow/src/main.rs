
fn foo(a:i32) {
  let X:i32 = a+100;

  if (X > a)
  {
      println!("No overflow\n");
  }
  else
  {
      println!("Overflow occured\n");
  }
  if (X > 0)
  {
      println!("No overflow\n");
  }
  else
  {
      println!("Overflow occured\n");
  }
  println!("{X} {a}");

}

fn main() {
  println!(">>> TEST 1 - Should not overflow\n");
  foo(100);
  println!(">>> TEST 2 - Should overflow\n");
  foo(0x7fffffff);
}



