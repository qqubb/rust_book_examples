/* An Example Program Using Structs

To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start by using single variables, and then refactor the program until we’re using structs instead. */

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "1. The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
	
	let rect1 = (30, 50);

    println!(
        "2. The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
	
	let rect1 = Rectangle {
	width: 30,
	height: 50,
    };

    println!(
        "3. The area of the rectangle is {} square pixels.",
        area_structs(&rect1) // main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
    );
	
	println!("rect1 is {:?}", rect1); 
	// Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
	// and 
	// outer attribute #[derive(Debug)] just before the struct definition
	
	// Another way to print out a value using the Debug format is to use the dbg! macro,
	let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call
	
	// Method Syntax
	let rect1 = Rectangle {
	width: 30,
	height: 50,
    };

    println!(
        "4. The area of the rectangle is {} square pixels.",
        rect1.area() // The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

	// Rust has a feature called automatic referencing and dereferencing
	// when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

	// p1.distance(&p2);
	// (&p1).distance(&p2);	
	
	// Methods with More Parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	
	// Associated Functions
	let sq = Rectangle::square(3); // To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example	
	
}


fn area(width: u32, height: u32) -> u32 {
    width * height
	
	/* The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related */
}

// Refactoring with Tuples

// Listing 5-9 shows another version of our program that uses tuples.

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
	
	// in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.
	
}

// Refactoring with Structs: Adding More Meaning

// We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts,

#[derive(Debug)] // Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_structs(rectangle: &Rectangle) -> u32 { // Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance

    rectangle.width * rectangle.height // conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1
	
}


// turning the area function into an area method defined on our Rectangle type.
// Method Syntax

// Methods are similar to functions: 
// we declare them with the fn keyword and a name, 
// they can have parameters and a return value, and 
// they contain some code that’s run when the method is called from somewhere else. 

// Unlike functions, methods are defined within the context of 
// a struct (or an enum or a trait object, 
// which we cover in Chapters 6 and 17, respectively), and 

// their first parameter is always self, 
// which represents the instance of the struct 
// the method is being called on.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
	fn width(&self) -> bool { 
		self.width > 0
    } // make the width method return true if the value in the instance’s width field is greater than 0

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

	// Methods with More Parameters

	// Let’s practice using methods by implementing a second method on the Rectangle struct. This time, we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle);

	// Associated Functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}



































































