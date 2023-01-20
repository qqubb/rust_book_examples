use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
		println!("Guess the number:");
		
		// first line, we call the rand::thread_rng function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system
		// Then we call the gen_range method on the random number generator
		// The gen_range method takes a range expression as an argument and generates a random number in the range.
		
		let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end and is inclusive on the lower and upper bounds
		
		// println!("The secret number is: {secret_number}");

		loop {
		
			println!("Input your guess.");
			
			let mut guess = String::new(); // To make a variable mutable, we add mut before the variable name:
			
			// The :: syntax in the ::new line indicates that new is an associated function of the String type. 
			// An associated function is a function that’s implemented on a type, in this case String. 
			// This new function creates a new, empty string. You’ll find a new function on many types, 
			// because it’s a common name for a function that makes a new value of some kind.
			
			// the let mut guess = String::new(); line has created 
			// a mutable variable 
			// that is currently bound to a new, 
			// empty instance of a String
					
			io::stdin() // we could still use the function by writing this function call as std::io::stdin
			// The stdin function returns an instance of std::io::Stdin, 
			// which is a type that represents a handle to the standard input for your terminal.
			
				.read_line(&mut guess)
				// The & indicates that this argument is a reference, which gives you a way to 
				// let multiple parts of your code access one piece of data 
				// without needing to copy that data into memory multiple times
				// references are immutable by default. 
				// Hence, you need to write &mut guess 
				// rather than &guess to make it mutable.
				.expect("Failed to read line.");
			
			// create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one
			
			// let guess: u32 = guess.trim().parse().expect("Please type a number");
			
			let guess: u32 = match guess.trim().parse() {
				Ok(num) => num,
				Err(_) => continue, // The underscore, _, is a catchall value;	
			};
			
			println!("You guessed: {guess}");
			
			// cmp - compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number.
			
			// use a match expression to decide what to do next based on which variant of Ordering was returned from the call
			
			match guess.cmp(&secret_number) {
				Ordering::Less => println!("Too small!"),
				Ordering::Greater => println!("Too big!"),
				Ordering::Equal => {
						println!("You win!");
						break;
				}
					
			}
			
			// A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all. These features will be covered in detail in Chapter 6 and Chapter 18, respectively.
		}
}

