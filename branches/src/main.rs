use std::io;

fn main() {
	// loop_label();
	// while_loop();
	// while_elements();
	// for_elements();
	// for_elements_rev();
	// convert_temp();
	// fibb(20);
	twelve_days();
}

fn loop_label() {
    let mut count = 0;
	
	// You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


// use while to loop the program three times, counting down each time, and then, after the loop, print a message and exit.

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Listing 3-3: Using a while loop to run code while a condition holds true
// this approach is error prone; we could cause the program to panic if the index value or test condition are incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.


fn while_elements() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// a more concise alternative, you can use a for loop and execute some code for each item in a collection
fn for_elements() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
	
	// Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array, as you would with the method used in Listing 3-4.

	// The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
}

// another method we’ve not yet talked about, rev, to reverse the range:
fn for_elements_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


// Convert temperatures between Fahrenheit and Celsius.

fn convert_temp() {
    println!("Convert temperatures between Fahrenheit and Celsius.");

    loop {
        println!("Please input temp in F");

        let mut f = String::new();

        io::stdin()
            .read_line(&mut f)
            .expect("Failed to read line");

        let f: f32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
		
		let c = (f-32.0)/1.8;

        println!("temp in C: {c}");
    }
	
}


// Generate the nth Fibonacci number.

fn fibb(mut count: i32) {
	
	let mut f0: u32 = 0;
	println!("{f0}");
	
	let mut f1: u32 = 1;
	println!("{f1}");
	
	let mut fx: u32 = 0;
	
	for number in (1..count) {
		
		fx = f0 + f1;
		
		println!("{fx}");
		
		f0 = f1;
		f1 = fx;
        }

}

// Iterator implementation
// https://www.umcconnell.net/posts/2021-03-13-fibonacci-rust/#4-iterator
// struct FibIterator {
    // a: usize,
    // b: usize
// }

// impl Default for FibIterator {
    // fn default() -> Self {
        // FibIterator { a: 1, b: 1 }
    // }
// }

// impl Iterator for FibIterator {
    // type Item = usize;

    // fn next(&mut self) -> Option<Self::Item> {
        // let curr = self.a;
        // self.a = self.b;
        // self.b = curr + self.a;

        // Some(curr)
    // }
// }

// Rust Iterators: Fibonacci series
// https://edgarluque.com/blog/rust-iterator-fibonacci/

// Some(number) of ways to calculate a Fibonacci Number in Rust 
// https://dev.to/jculverhouse/some-number-of-ways-to-calculate-a-fibonacci-number-in-rust-d78

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn twelve_days() {
	let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

	let gifts: [&str; 12] = ["a Partridge in a Pear Tree\n", "Two Turtle Doves", "Three French Hens", "Four Calling Birds", "Five Golden Rings", "Six Geese a Laying", "Seven Swans a Swimming", "Eight Maids a Milking", "Nine Ladies Dancing", "Ten Lords a Leaping", "Eleven Pipers Piping", "12 Drummers Drumming"];
    
	let mut printed_days = 0;
	println!("On the {} day .... {}", days[0], gifts[0]);
	
	'counting_up: loop {

		for day in (1..printed_days).rev() {
			
			println!("On the {} day .... {}", days[day], gifts[day]);

			if day == 1 {
				println!("AND {}", gifts[0])
			};
		}		
		
		printed_days += 1;

		if printed_days == days.len()+1 {
                break 'counting_up;
		};
		
		
	}
	
} 



























