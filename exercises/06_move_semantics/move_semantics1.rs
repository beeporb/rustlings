// TODO: Fix the compiler error in this function.
// Solution
// Make the shadowed vec mutable.
// If you then try to use vec0, you'll get an error because vec0 has been "moved".
// This is because you have provided the value to the function by Value.
// Because Vec does not implement Copy, it is instead Moved to the function, where
// ownership is taken.
// If Vec did implement Copy, then the compiler would see this and the value would be
// copied. You'd then still be able to use the old version vec[0].
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.

    let vec0: Vec<i32> = vec![22,44,66];
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}