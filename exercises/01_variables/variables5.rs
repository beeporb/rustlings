fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // Solution
    // Shadowing means that we can define a variable with let.
    let number = 3;
    println!("Number plus two is: {}", number + 2);

    let number: i32 = 5;
    println!("Number: {}", number)
}
