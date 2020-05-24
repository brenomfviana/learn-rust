use std::fmt;
use std::ops::Add;

//# Advanced Traits

// Operator overloading
#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  // Specifying Placeholder Types in Trait Definitions with Associated Types
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point { x: self.x + other.x, y: self.y + other.y, }
  }
}

// Operator overloading with custom RHS
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}


// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same
// Name

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) { println!("This is your captain speaking."); }
}

impl Wizard for Human {
  fn fly(&self) { println!("Up!"); }
}

impl Human {
  fn fly(&self) { println!("*waving arms furiously*"); }
}

// Functions without self
trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String { String::from("Spot") }
}

impl Animal for Dog {
  fn baby_name() -> String { String::from("puppy") }
}


// Using Supertraits to Require One Trait’s Functionality Within Another Trait

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

// Using the Newtype Pattern to Implement External Traits on External Types

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}


//# Advanced Types

// Creating Type Synonyms with Type Aliases

type Kilometers = i32;

// type Result<T> = std::result::Result<T, std::io::Error>;

// The Never Type that Never Returns

fn _bar() -> ! { panic!("bar") }


//# Advanced Functions and Closures

// Function Pointers

fn add_one(x: i32) -> i32 { x + 1 }

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { f(arg) + f(arg) }

#[derive(Debug)]
enum Status {
  Value(u32),
  Stop,
}

// Returning Closures

fn returns_closure() -> Box<dyn Fn(i32) -> i32> { Box::new(|x| x + 1) }


//# Macros

// Declarative Macros with macro_rules! for General Metaprogramming
#[macro_export]
macro_rules! vec_thor {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $( temp_vec.push($x); )*
      temp_vec
    }
  };
}

// How to Write a Custom derive Macro

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// Attribute-like macros




// Main

fn main() {
  // Operator overloading
  assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
  // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same
  // Name
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();
  println!("A baby dog is called a {}", Dog::baby_name());
  // println!("A baby dog is called a {}", Animal::baby_name()); // Don't work
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // Work
  // Using Supertraits to Require One Trait’s Functionality Within Another Trait
  Point { x: 1, y: 0 }.outline_print();
  // Using the Newtype Pattern to Implement External Traits on External Types
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
  //
  // Creating Type Synonyms with Type Aliases
  let x: i32 = 5;
  let y: Kilometers = 5;
  println!("x + y = {}", x + y);
  //
  // Function Pointers
  let answer = do_twice(add_one, 5);
  println!("The answer is: {}", answer);
  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();
  // let list_of_strings: Vec<String> =
  //       list_of_numbers.iter().map(|i| i.to_string()).collect();
  println!("{:?}", list_of_strings);
  let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
  println!("{:?}", list_of_statuses);
  println!("{:?}", returns_closure()(10));
  //
  // Macros
  let list_of_numbers = vec_thor![1, 2, 3];
  println!("{:?}", list_of_numbers);
  Pancakes::hello_macro();
}
