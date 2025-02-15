#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}


#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    // Soluton
    // Define the enumeration again, this time with shapes as described by the call to the
    // enum.
    Resize {
        width: i32,
        height: i32
    },
    Move(Point),
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
