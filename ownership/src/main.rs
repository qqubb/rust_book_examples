fn main() {
	
	let s = String::from("hello");
	println!("{}", first_word(&s));
	
	// String Slices

	let s = String::from("hello world");
	
	let hello = &s[0..5];
    let world = &s[6..11];	

	println!("{}", hello);
	println!("{}", world);
	
	println!("{}", first_word2(&s));
	
	println!("{}", first_word3(&s));
	
	let a = [1, 2, 3, 4, 5];

	let slice = &a[1..3];

	assert_eq!(slice, &[2, 3]);
	
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// rewrite first_word to return a slice
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

	// When we find a space, we return a string slice using 
	// the start of the string and 
	// the index of the space as 
	// the starting and ending indices.

    &s[..]
}

// String Literals Are Slices

// Recall that we talked about string literals being stored inside the binary. 
// Now that we know about slices, 
// we can properly understand string literals:

// let s = "Hello, world!";

// The type of s here is &str: it’s a 
// slice pointing to that specific point of the binary. 
// This is also why string literals are immutable; 
// &str is an immutable reference.

/* String Slices as Parameters

Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

fn first_word(s: &String) -> &str {

A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

fn first_word(s: &str) -> &str {

Listing 4-9: Improving the first_word function by using a string slice for the type of the s parameter */

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}