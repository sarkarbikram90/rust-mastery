enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    Some(T),
    None,
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _home = IpAddr {
        kind: home,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        kind: loopback,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_chars = Some('e');

    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = match y {
    //     Some(value) => x + value,
    //     None => x,
    // };

    // println!("sum = {}", sum);
}
