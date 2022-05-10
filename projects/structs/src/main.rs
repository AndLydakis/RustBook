struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("test@test.com"),
        username: String::from("test test"),
        active: true,
        sign_in_count: 1,
    };

    let t1 = String::from("user2");
    let t2 = String::from("test@test2.com");
    let user2 = build_user(t1, t2);

    println!("User email: {}", user1.email);
    println!("User2 email: {}", user2.email);

    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Rect1: {:?}", rect1);
    // println!("The area of the rectangle is {}", area(&rect1));
    println!("The area of the rectangle is {}", rect1.area());

    // let scale = 2;
    // let rect2 = Rectangle{
    //     width: dbg!(30*scale),
    //     height: 50
    // };
    dbg!(&rect2);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(r: &Rectangle) -> u32 {
    r.height * r.width
}
