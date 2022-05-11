enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5); //Option<i32>
    let some_string = Some("a string"); // Option<&str>
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Not valid
    // let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", five, six);

    // catch-all matches
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // same as
        //_ => reroll(), empty function
        //_ => (), do nothing
    }

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    // let state = State::Alabama;
    let coin = Coin::Quarter(UsState::Alabama);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // Equivalent to
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
