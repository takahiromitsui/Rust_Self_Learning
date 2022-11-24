use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
};

use unicode_segmentation::UnicodeSegmentation;

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
    // chapter 8
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);
    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // chapter 8 vector loop
    let mut numbers = vec![1, 2, 3, 4, 5];
    for i in &mut numbers {
        *i += 50;
    }
    for i in &numbers {
        println!("{}", i);
    }
    // chapter 8 vector enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    match &row[0] {
        SpreadsheetCell::Text(s) => println!("{}", s),
        _ => (),
    }
    // chapter 8 String
    let hello = String::from("Hello");
    for b in hello.chars() {
        println!("{}", b);
    }
    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // chapter 8 hash map
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut map = HashMap::new();
    map.insert(blue, 10);
    map.insert(yellow, 50);
    // println!("{}", blue)
    let team_name = String::from("blue");
    let score = map.get(&team_name);
    println!("{:?}", score);
    // chapter9 panic
    // let v = vec![1, 2, 3];
    // v[99];
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem opening the file: {:?}", error),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    let usernames = read_username_from_file().unwrap();
    println!("{}", usernames);
    // chapter 10
    let tweet = Tweet {
        username: String::from("@janedoe"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        author: String::from("Jane Doe"),
        headline: String::from("Ein Prosit"),
        content: String::from("Ein Prosit, ein Prosit, der Gemütlichkeit"),
    };
    // println!("1 new tweet: {}", tweet.summarize());
    // println!("1 new article: {}", article.summarize());
    notify(&tweet, &article);
    // chapter 10 lifetime
    let message1 = "Hello!";
    let message2 = "Guten Tag!";
    let longest = longest(message1, message2);
    println!("The longest message is {}", longest);
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

// chapter 9 error handling
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut f = match f_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
    fs::read_to_string("hello.txt")
}

// chapter 10 trait
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
