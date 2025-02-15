#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    // Solution
    // Go ahead and actually define some enum values,
    // as described in https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html?highlight=enum#enum-values
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
