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
fn main() {
    let user1 = User {
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
}
