struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// chapter 6
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum WinstonLevel {
    Error,
    Warn,
    Info,
    Http,
    VERBOSE,
    Debug,
    Silly,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    //chapter 6
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    let some_coin = Coin::Penny;
    println!("Value in cents: {}", value_in_cents(some_coin));
    // enum experiment
    custom_logger(WinstonLevel::Error, "Something went wrong");

    // chapter 6-placeholder
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn custom_logger(level: WinstonLevel, message: &str) {
    match level {
        WinstonLevel::Error => println!("Error: {}", message),
        WinstonLevel::Warn => println!("Warn: {}", message),
        WinstonLevel::Info => println!("Info: {}", message),
        WinstonLevel::Http => println!("Http: {}", message),
        WinstonLevel::VERBOSE => println!("VERBOSE: {}", message),
        WinstonLevel::Debug => println!("Debug: {}", message),
        WinstonLevel::Silly => println!("Silly: {}", message),
    }
}

// chapter 6-placeholder
fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}
