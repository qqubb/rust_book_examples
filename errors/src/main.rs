
// Matching on Different Errors
// Listing 9-5: Handling different kinds of errors in different ways

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
        // Ok(file) => file,
        // Err(error) => match error.kind() {
            // ErrorKind::NotFound => match File::create("hello.txt") {
                // Ok(fc) => fc,
                // Err(e) => panic!("Problem creating the file: {:?}", e),
            // },
            // other_error => {
                // panic!("Problem opening the file: {:?}", other_error);
            // }
        // },
    // };
// }

// Alternatives to Using match with Result<T, E>
// another way to write the same logic as shown in Listing 9-5, 
// this time using closures and the unwrap_or_else method:

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Shortcuts for Panic on Error: unwrap and expect
// most Rustaceans choose expect rather than unwrap and 
// give more context about why the operation is expected to always succeed.

// use std::fs::File;

// fn main() {
    // let greeting_file = File::open("hello.txt")
        // .expect("hello.txt should be included in this project");
// }

// Listing 9-9: Using fs::read_to_string instead of opening and then reading the file

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

