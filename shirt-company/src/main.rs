#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// Listing 13-1: Shirt company giveaway situation

// The store defined in main has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion. We call the giveaway method for a user with a preference for a red shirt and a user without any preference.

// Again, this code could be implemented in many ways, and here, to focus on closures, we’ve stuck to concepts you’ve already learned except for the body of the giveaway method that uses a closure. In the giveaway method, 
// we get the user preference as a parameter of type Option<ShirtColor> and 
// call the unwrap_or_else method on user_preference. 
// The unwrap_or_else method on Option<T> is defined by the standard library. It takes one argument: a closure without any arguments that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor). 
// If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. 
// If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.

// We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else. This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars). 
// The body of the closure calls self.most_stocked(). We’re defining the closure here, and the implementation of unwrap_or_else will evaluate the closure later if the result is needed.

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
