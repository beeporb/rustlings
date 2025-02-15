// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// use ???;
// use std::time::UNIX_EPOCH;
// use std::time::SystemTime;

// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#:~:text=Using%20Nested%20Paths%20to%20Clean%20Up%20Large%20use%20Lists&text=Instead%2C%20we%20can%20use%20nested,shown%20in%20Listing%207%2D18.
use std::time::{UNIX_EPOCH, SystemTime};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
