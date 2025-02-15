fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![22,44,66];
    let vec0_copy = vec0.clone();
    let vec1 = fill_vec(vec0_copy);

    // Doing something like this also doesn't work.
    // When we assign vec0_copy we're actually borrowing vec0.
    // This therefore means that vec0[0] would be attempting to access
    // the value after it has been borrowed.
    // We would need to use .clone() in this case too.
    println!("{}", vec0[0]);
    println!("{}", vec1[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    // Solution
    // You can use the .clone() function on a vector to return a copy of it.
    // This means that both vec0 and vec1 are now pointing to different data.
    // Passing vec0.clone() into the function isn't passing the value to be copied,
    // but a clone of the value.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
