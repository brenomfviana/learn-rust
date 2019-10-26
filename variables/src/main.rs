const MAX_POINTS: u32 = 100_000;

//-- Variables are immutable by default
// fn main() {
//   let x = 5;
//   println!("The value of x is: {}", x);
//   x = 6;
//   println!("The value of x is: {}", x);
// }

//-- But we can turn them mutable by using `mut`
fn mutability() {
  let mut x = 5;
  println!("The value of x is: {}.", x);
  x = 6;
  println!("The value of x is: {}.", x);
}

//-- What's the difference between immutable variables and conttants?
//-- Variables can be shadowed
//-- - By using let, we can perform a few transformations on a value but have
//--   the variable be immutable after those transformations have been
//--   completed.
//-- - The other difference between mut and shadowing is that because we’re
//--   effectively creating a new variable when we use the let keyword again, we
//--   can change the type of the value but reuse the same name.
fn shadowing() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}.", x);
  let spaces = "   ";
  let spaces = spaces.len();
  //-- We’re not allowed to mutate a variable’s type:
  // let mut spaces = "   ";
  // spaces = spaces.len();
  println!("Spaces: {}.", spaces);
}

fn main() {
  mutability();
  shadowing();
  println!("Constant: {}.", MAX_POINTS);
}
