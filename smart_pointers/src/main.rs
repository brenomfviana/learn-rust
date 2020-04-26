mod messager;

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::List::{Cons, Nil};
use crate::SList::{SCons, SNil};
use crate::SMList::{SMCons, SMNil};
use crate::AList::{ACons, ANil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &T { &self.0 }
}

#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}

// Shared reference list
#[derive(Debug)]
enum SList {
  SCons(i32, Rc<SList>),
  SNil,
}

// Shared and mutate reference list
#[derive(Debug)]
enum SMList {
  SMCons(Rc<RefCell<i32>>, Rc<SMList>),
  SMNil,
}

#[derive(Debug)]
enum AList {
  ACons(i32, RefCell<Rc<AList>>),
  ANil,
}

impl AList {
  fn tail(&self) -> Option<&RefCell<Rc<AList>>> {
    match self {
      ACons(_, item) => Some(item),
      ANil => None,
    }
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
  // Recursive list
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("list = {:?}", list);
  let a = Rc::new(SCons(5, Rc::new(SCons(10, Rc::new(SNil)))));
  println!("a = {:?}", a);
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = SCons(3, Rc::clone(&a));
  println!("b = {:?}", b);
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = SCons(4, Rc::clone(&a));
    println!("c = {:?}", c);
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
  //
  // Derefering
  let x = 5;
  let y = &x;
  assert_eq!(5, x);
  assert_eq!(5, *y);
  let x = 5;
  let y = Box::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);
  //
  // Custom Box
  let x = 5;
  let y = MyBox::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);
  //
  // Deref coercion
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
  //
  // Without deref coercion
  let m = MyBox::new(String::from("Rust"));
  hello(&(*m)[..]);
  //
  // Custom Drop
  let c = CustomSmartPointer { data: String::from("my stuff"), };
  let _d = CustomSmartPointer { data: String::from("other stuff"), };
  println!("CustomSmartPointers created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
  //
  // Rc and RefCell
  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(SMCons(Rc::clone(&value), Rc::new(SMNil)));
  let b = SMCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = SMCons(Rc::new(RefCell::new(10)), Rc::clone(&a));
  *value.borrow_mut() += 10;
  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
  //
  // Reference Cycles Can Leak Memory
  let a = Rc::new(ACons(5, RefCell::new(Rc::new(ANil))));
  println!("a initial rc count = {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());
  let b = Rc::new(ACons(10, RefCell::new(Rc::clone(&a))));
  println!("a rc count after b creation = {}", Rc::strong_count(&a));
  println!("b initial rc count = {}", Rc::strong_count(&b));
  println!("b next item = {:?}", b.tail());
  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }
  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));
  // Uncomment the next line to see that we have a cycle;
  // it will overflow the stack
  // println!("a next item = {:?}", a.tail());
  //
  // Avoiding reference cycles
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });
  println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf),
    Rc::weak_count(&leaf));
  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch),
      Rc::weak_count(&branch));
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf),
      Rc::weak_count(&leaf));
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
  }
  println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
  println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),);
}
