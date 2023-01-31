use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    
    // Listing 19-17: Calling fly on an instance of Human
    let person = Human;
    person.fly();
    
    // Listing 19-18: Specifying which trait’s fly method we want to call
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    
    // We could also write Human::fly(&person), which is equivalent to the person.fly() 
    Human::fly(&person);

    // Listing 19-19
    println!("A baby dog is called a {}", Dog::baby_name());    
    
    // Listing 19-21: Using fully qualified syntax to specify that 
    // we want to call the baby_name function from the Animal trait as implemented on Dog
    // We’re providing Rust with a type annotation within the angle brackets
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); 

    // Listing 19-22
    let p = Point { x: 1, y: 3 };
    p.outline_print();
    
    let q = Point { x: 100000, y: 300000 };
    q.outline_print();

    // Listing 19-23: Creating a Wrapper type around Vec<String> to implement Display            
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);            
            
} // Listing 19-14: Implementing the Add trait to overload the + operator for Point instances


trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
} // Listing 19-16: Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
} // Listing 19-19: A trait with an associated function and a type with an associated function of the same name that also implements the trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
} // Listing 19-22: Implementing the OutlinePrint trait that requires the functionality from Display

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
} // Listing 19-22



struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
} // Listing 19-23: Creating a Wrapper type around Vec<String> to implement Display






























































