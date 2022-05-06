fn main() {
    // let mut x = 5;
    // println!("value of x: {}", x);
    // x = 6;
    // println!("value of x: {}", x);

    // SHADOWING
    //     let x = 5;
    //     let x = x + 1;
    //     {
    //         let x = x * 2;
    //         println!("The value of x in the inner scope is: {}", x);
    //     }
    //     println!("The value of x is: {}", x);

    // will work
    // let spaces = "   ";
    // let spaces = spaces.len();

    // won't work
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // DATA TYPES
    // throws error
    // let guess: u32 = "42".parse().expect("Not a number!");
    // works
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup_ = (500, 6.4, 1);
    let (tx, ty, tz) = tup;
    println!("The value of ty is {}", ty);

    let x_: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x_.0;
    let six_point_four = x_.1;
    let one = x_.2;

    // ARRAYS
    let ar = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let ar_fixed_type_size: [i32; 5] = [1, 2, 3, 4, 5];

    let three_fives = [3; 5];

    another_function(5);

    print_labeled_measurements(15, 'h');

    println!("{}", five());

    let mut number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    number = 3;

    // ERROR
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // will return this value
        }
    };

    println!("The result is {}", result);

    number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("==============");
    for element in a {
        println!("the value is: {}", element);
    }
    println!("==============");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x: {}", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label)
}

fn five() -> i32 {
    5
}
