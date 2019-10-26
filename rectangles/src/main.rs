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
}
