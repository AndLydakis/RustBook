use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    {
        let v = vec![1, 2, 3];
    } // v gets dropped here

    let v5 = vec![1, 2, 3];
    let third: &i32 = &v5[2];
    println!("Third element: {}", third);

    match v5.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element"),
    }

    let v6 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v6[100]; will cause crash
    // let does_not_exist = v6.get(100); will return none

    let mut v7 = vec![1, 2, 3, 4, 5];
    let first = &v7[0];
    // v7.push(6); will cause a crash, vectors are dynamically allocated
    println!("The first element is: {}", first);

    let mut v8 = vec![100, 32, 57];
    for i in &v8 {
        println!("{}", i);
    }

    for i in &mut v8 {
        *i *= 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // STRINGS
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");

    let mut s_ = String::from("foo ");
    s_.push_str("bar");
    s_.push('!');
    println!("{}", s_);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Indexing into string / Slicing strings not allowed

    // let s1 = String::from("hello");
    // let h = s1[0];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // hashmaps

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // This code will print each pair in an arbitrary order:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
