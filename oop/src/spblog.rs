/// Blog draft state.
struct Draft {}

/// Blog first pending review state.
struct FirstPendingReview {}

/// Blog second pending review state.
struct SecondPendingReview {}

/// Blog published state.
struct Published {}

/// Blog post state.
trait State {
  /// Requests the review of the blog post.
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  /// Approves the blog post.
  fn approve(self: Box<Self>) -> Box<dyn State>;
  /// Rejects the blog post.
  fn reject(self: Box<Self>) -> Box<dyn State>;
  /// Returns the blog post content.
  fn content<'a>(&self, _post: &'a Post) -> &'a str { "" }
  /// Adds text to the blog post content.
  fn add_text(&self, _content: &str) -> String { String::new() }
}

impl State for Draft {
  /// Requests the review of the blog post.
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(FirstPendingReview {})
  }

  /// Ignores approvation because this is a blog draft.
  fn approve(self: Box<Self>) -> Box<dyn State> { self }

  /// Ignores rejection because this is a blog draft.
  fn reject(self: Box<Self>) -> Box<dyn State> { self }

  /// Adds text to the blog post content.
  fn add_text(&self, content: &str) -> String { String::from(content) }
}

impl State for FirstPendingReview {
  /// Requests another review of the blog post.
  fn request_review(self: Box<Self>) -> Box<dyn State> { self }

  /// Approves the blog post publication.
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(SecondPendingReview {})
  }

  /// Rejects the blog post.
  fn reject(self: Box<Self>) -> Box<dyn State> { Box::new(Draft {}) }
}

impl State for SecondPendingReview {
  /// Requests another review of the blog post.
  fn request_review(self: Box<Self>) -> Box<dyn State> { self }

  /// Approves the blog post publication.
  fn approve(self: Box<Self>) -> Box<dyn State> { Box::new(Published {}) }

  /// Rejects the blog post.
  fn reject(self: Box<Self>) -> Box<dyn State> { Box::new(Draft {}) }
}

impl State for Published {
  /// Ignores review because this blog post was already published.
  fn request_review(self: Box<Self>) -> Box<dyn State> { self }

  /// Ignores approvation because this blog post was already published.
  fn approve(self: Box<Self>) -> Box<dyn State> { self }

  /// Ignores rejection because this blog post was already published.
  fn reject(self: Box<Self>) -> Box<dyn State> { self }

  /// Returns the blog post content.
  fn content<'a>(&self, post: &'a Post) -> &'a str { &post.content }
}

/// Blog post.
pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  /// Creates a blog post draft.
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  /// Adds some text to a blog post.
  pub fn add_text(&mut self, text: &str) {
    self.content = self.state.as_ref().unwrap().add_text(text);
  }

  /// Gets blog post content.
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }

  /// Requests the review of the blog post.
  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  /// Approves the blog post.
  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}
