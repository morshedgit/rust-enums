// Definition
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
        println!("Something");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny =>1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter =>25
    }
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127,0,0,0);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

}
