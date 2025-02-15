fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    // Solution
    // A mutable reference is only "active" for as long as it is used
    // in the code.
    // We see in the original version of this code that z is immediately created
    // after y. But this reference is being created before the former is actually used
    // (we make use of y in the line y.push()). This cannot be the case.
    // We have to move the code for using y above the creation of the new reference.
    // Because we have no further code that uses y, it's inactive and therefore a new reference
    // can be made to x.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
