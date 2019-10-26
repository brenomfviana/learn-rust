/// This program calculates the area of rectangles by using different ways.
/// This code was based on the follow chapter:
///   https://doc.rust-lang.org/book/ch05-02-example-structs.html

mod rect;

fn main() {
  // By using separeted values
  let width1 = 30;
  let height1 = 50;
  println!("The area of the rectangle is {} square pixels.",
    rect::area_v1(width1, height1));

  // By using tuple
  let rect1 = (30, 50);
  println!("The area of the rectangle is {} square pixels.",
    rect::area_v2(rect1));

  // By using struct
  let rect1 = rect::Rectangle{ width: width1 as u32, height: height1 as u32 };
  println!("The area of the rectangle is {} square pixels.",
    rect::area_v3(&rect1));
  println!("rect1 is {:#?}", rect1);

  // By using struct and methods
  let rect1 = rect::Rectangle{ width: width1 as u32, height: height1 as u32 };
  println!("The area of the rectangle is {} square pixels.", rect1.area());
  println!("rect1 is {:#?}", rect1);

  // Another method
  let rect1 = rect::Rectangle{ width: 30, height: 50 };
  let rect2 = rect::Rectangle{ width: 10, height: 40 };
  let rect3 = rect::Rectangle{ width: 60, height: 45 };

  println!("Can rect1 hold rect2? {}.", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}.", rect1.can_hold(&rect3));

  // Using an associated function for create a square
  let sq = rect::Rectangle::square(3);
  println!("The area of the square is {} square pixels.", sq.area());
}
