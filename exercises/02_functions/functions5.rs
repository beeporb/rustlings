// TODO: Fix the function body without changing the signature.
// Solution
// Remove the semi-colon to make the line an expression, which in turn makes the
// whole function resolve to num * num.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
