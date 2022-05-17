struct Point<T> {
    x: T,
    y: T,
}

struct PointV2<T, U> {
    x: T,
    y: U,
}

fn main() {
    // let number_list = vec![32, 50, 25, 100, 65];

    // let mut res = number_list[0];

    // for number in number_list {
    //     if number > res {
    //         res = number;
    //     }
    // }

    // println!("The largest number is {}", res);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let res = largest(&number_list);
    // println!("The largest number is {}", res);

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // won't work
    // let wont_work = Point{x:5, y: 3.0};

    let both_integer = PointV2 { x: 5, y: 10 };
    let both_float = PointV2 { x: 1.0, y: 4.0 };
    let integer_and_float = PointV2 { x: 5, y: 4.0 };
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
