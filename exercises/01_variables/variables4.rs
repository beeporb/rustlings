// TODO: Fix the compiler error.
// Solution:
// X is being assigned multiple times despite being immutable.
// Must use the *mut* keyword to enable mutability.
fn main() {
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
