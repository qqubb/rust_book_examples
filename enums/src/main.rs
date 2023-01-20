
// Enums and Pattern Matching

// In this chapter we’ll look at enumerations, also referred to as enums. 
// Enums allow you to define a type by enumerating its possible variants. 

// First, we’ll define and use an enum to show how an enum can encode meaning along with data. 

// Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. 

// Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum. 

// Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.

// enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle


// enum IpAddrKind {
    // V4,
    // V6,
// }

fn main() {
	
	// We can create instances of each of the two variants of IpAddrKind like this:
	
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
	// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

	// rather than an enum inside a struct, we can put data directly into each enum variant	
	// enum IpAddr {
		// V4(String),
		// V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));
	
	// the name of each enum variant that we define also becomes a function that constructs an instance of the enum. 
	// That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type
	
	
	// another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. Version four type IP addresses will always have four numeric components that will have values between 0 and 255.
	
	// enum IpAddr {
        // V4(u8, u8, u8, u8),
        // V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);

    // let loopback = IpAddr::V6(String::from("::1"));

	// we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.
	let m = Message::Write(String::from("hello"));
    m.call();
}


// We can then, for instance, define a function that takes any IpAddrKind:
// fn route(ip_kind: IpAddrKind) {}

// another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants.
// Listing 6-2: A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


// just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. 
impl Message {
	fn call(&self) {
		// method body would be defined here
	}
}

// The Option Enum and Its Advantages Over Null Values
// The Option type encodes the very common scenario in which a value could be something or it could be nothing

// Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent

enum Option<T> { // The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter
    None,
    Some(T),
}

// some examples of using Option values to hold number types and string types:

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;