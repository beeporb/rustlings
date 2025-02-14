fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    // Solution:
    // add value to be passed to call_me
    call_me(7);
}
