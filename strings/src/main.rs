fn main() {
	
	// Creating a new, empty String
    let mut s = String::new(); // creates a new empty string called s, which we can then load data into
	
	// Listing 8-12: Using the to_string method to create a String from a string literal
	let data = "initial contents";

    // use the to_string method to load initial data
	let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
	
	// use the function String::from to create a String from a string literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12 that uses to_string.
	let s = String::from("initial contents");
	
	// Appending to a String with push_str and push
	let mut s = String::from("foo");
    s.push_str("bar");
	
	// Using a string slice after appending its contents to a String
	// use s2 after appending its contents to s1.
	let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
	
	// Listing 8-17: Adding one character to a String value using push
	// The push method takes a single character as a parameter and adds it to the String. Listing 8-17 adds the letter “l” to a String using the push method.
	let mut s = String::from("lo");
    s.push('l');
	println!("s is {}", &s);
	
	// Listing 8-18: Using the + operator to combine two String values into a new String value
	// Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
	println!("s is {}", s3);
	
	
	// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
	
	// we can instead use the format! macro:
	let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
	
	// Methods for Iterating Over Strings
	// For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:
	for c in "Зд".chars() {
		println!("{}", c);
	}
	
	// Alternatively, the bytes method returns each raw byte	
	for b in "Зд".bytes() {
		println!("{}", b);
	}
	
}