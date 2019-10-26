/// This module contains functions that calculate the area of a rectangle by
/// getting different arguments.

/// This struct defines a rectangle
#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

/// This function calculates the area of a rectangle
pub fn area_v1(width: u32, height: u32) -> u32 {
  width * height
}

/// This function calculates the area of a rectangle
pub fn area_v2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

pub fn area_v3(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
