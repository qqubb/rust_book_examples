// The match Control Flow Construct

// Rust has an extremely powerful control flow construct called match that 

	// allows you to compare a value against a series of patterns and 
	// then execute code based on which pattern matches. 

// Patterns can be made up of literal values, variable names, wildcards, and many other things; 

// Chapter 18 covers all the different kinds of patterns and what they do. 

// The power of match comes from the expressiveness of the patterns and 
// the fact that the compiler confirms that all possible cases are handled.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
	
	// break down the match in the value_in_cents function:
	
	// First, we list the match keyword followed by an expression, 
	// which in this case is the value coin. 
	
	// This seems very similar to an expression used with if, 
	// but there’s a big difference: 
		// with if, the expression needs to return a Boolean value, 
			// but here, it can return any type. 
	// The type of coin in this example 
	// is the Coin enum that we defined on the first line.	
		
	// Next are the match arms. 
	// An arm has two parts: a pattern and some code. 
	
	// The first arm here has a pattern that is the value Coin::Penny and 
	// then the => operator that separates 
		// the pattern and the code to run. 
	// The code in this case is just the value 1. 
	// Each arm is separated from the next with a comma.
		
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
	

}

// the following code prints “Lucky penny!” every time the method is called with a Coin::Penny, but still returns the last value of the block, 1:

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}




fn main() {
	
	// Matching with Option<T>

	// In the previous section, we wanted to get the inner T value out of the Some case when using Option<T>; we can also handle Option<T> using match as we did with the Coin enum! Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.

	// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

	// This function is very easy to write, thanks to match, and will look like Listing 6-5.
	
	fn plus_one(x: Option<i32>) -> Option<i32> {
		match x {
			None => None,
			Some(i) => Some(i + 1),
		}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
	
}