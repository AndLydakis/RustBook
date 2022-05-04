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
}
