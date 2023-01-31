// Advanced Functions and Closures

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
} // The do_twice function calls the function f twice, passing it the arg value, then adds the two function call results together

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
} // Listing 19-27: Using the fn type to accept a function pointer as an argument
