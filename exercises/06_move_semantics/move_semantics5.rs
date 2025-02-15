#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
// Solution
// As this function shouldn't take ownership of the name,
// it should expect to be passed the value by reference using  &.
// We then need to change the call to this function
// to pass in the reference.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// Solution
// As this function should take ownership of the value,
// we should pass it in by value.
// The value can be made mutable as the function is going to change
// the string to upper case.
// Note because the function doesn't return the value of data
// it is only accessible within this function's scope and then
// no longer.
// Trying to access data after the function has been called should invoke
// an error in the compiler as the value was borrowed and never returned.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);

    // This will now fail, because string_uppercase borrowed the value
    // and did not return  it.
    // println!("{data}");
}
