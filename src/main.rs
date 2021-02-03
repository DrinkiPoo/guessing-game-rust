use std::io;

fn main() {
	println!("Guess a number!");
	println!("Please enter your guess: ");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
		.ok()
		.expect("Failed to read line");

	println!("You guessed: {}", guess);
}
