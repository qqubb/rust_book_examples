fn main() {
	
	let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
	
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // Using & and [] gives us a reference to the element at the index value
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2); // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
	
	// Iterating over the Values in a Vector
	let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
	
	// iterate over mutable references to each element in a mutable vector
	let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // we have to use the * dereference operator to get to the value in i before we can use the += operator
		println!("{}", i);
    }
	
	
	// Using an Enum to Store Multiple Types
	// define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum
	#[derive(Debug)]
	enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
	
    for i in row {
		println!("{:?}", i);
    }

}