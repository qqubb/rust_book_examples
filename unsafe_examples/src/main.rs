/*
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

// Listing 19-1: Creating raw pointers from references
*/

/*fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// Listing 19-3: Dereferencing raw pointers within an unsafe block
*/

/*
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("a is: {:?}", a);
    println!("b is: {:?}", b);    
    
}

// Listing 19-4: Using the safe split_at_mut function

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    // We use the len method to get the length of a slice and 
    // the as_mut_ptr method to access the raw pointer of a slice. 
    // In this case, because we have a mutable slice to i32 values, 
    // as_mut_ptr returns a raw pointer with the type *mut i32, 
    // which we’ve stored in the variable ptr.

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Listing 19-6: Using unsafe code in the implementation of the split_at_mut function
*/

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Listing 19-8: Declaring and calling an extern function defined in another language
