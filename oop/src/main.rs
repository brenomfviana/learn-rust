mod spblog;
mod blog;

fn main() {
  // Blog implemented with state pattern
  let mut post = spblog::Post::new();
  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());
  post.request_review();
  assert_eq!("", post.content());
  post.approve();
  assert_eq!("", post.content());
  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
  // Blog with structs
  let mut post = blog::Post::new();
  post.add_text("I ate a salad for lunch today");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}
