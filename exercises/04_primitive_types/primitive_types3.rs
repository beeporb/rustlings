fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    // Solution
    // https://doc.rust-lang.org/book/ch03-02-data-types.html#:~:text=as%20shown%20here%3A-,let%20a%20%3D%20%5B3%3B%205%5D%3B,-The%20array%20named
    // creates an array of 101 elements, all set to 1.
    let a = [1; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
