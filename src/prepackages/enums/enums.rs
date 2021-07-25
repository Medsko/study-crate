use super::matching;
use super::options;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

enum ConciseIpAddr {
    V4(u8, u8, u8, u8), // Dit is gewoon helemaal prima
    V6(String),
}

#[derive(Debug)]
pub enum Message {  // Iedere variant van een enum krijgt zn eigen Struct definitie, dus:
Quit,                       // - geen velden (Unit struct)
Move { x: i32, y: i32 },    // - named fields (Waaaaaaat)
Write(String),              // - tuple Struct
ChangeColor(i32, i32, i32), // - (idem)
    // ...zijn allemaal valide
}

impl Message {
    fn call(&self) {
        // Do some pattern matched stuff specific for this enum variant
        match self {
            Message::Write(message) => println!("{}", message),
            Message::Move{x, y} => println!("Moving {} up and {} right", x, y),
            Message::ChangeColor(r, g, b) => println!("Changing rgb ({},{},{})", r, g, b),
            _ => () // In all other cases: do nothing
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enumerate() {

    let four = IpAddrKind::V4;
    let hip = IpAddrKind::V6;

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: "::1".to_string(),
    };

    let concise_loopback = ConciseIpAddr::V6(String::from("::1"));

    let home = ConciseIpAddr::V4(127, 0, 0, 1);

    let message_to_write = Message::Write("I want to write this message".to_string());
    message_to_write.call();

    let move_message = Message::Move{ x: 14, y: 15};
    move_message.call();

    options::options();

    matching::match_stuff();

}

