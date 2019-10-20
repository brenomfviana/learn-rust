use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number (between 1 and 100)!");
  // Choose a random number between 1 and 101 (excluding)
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    println!("Please input your guess.");
    // Get the user guess
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("You guessed: {}", guess);
    // Convert to u32 if the entered value is a number or ask again
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    // Check the user guess
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
