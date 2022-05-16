use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt"); // OK
                                     // let f: u32 = File::open("hello.txt"); NOT OK

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file {:?}", error),
    // };

    // let f = match f{
    //     Ok(file)=>file,
    //     Err(error)=>match error.kind(){
    //         ErrorKind::NotFound=>match File::create("hello.txt"){
    //             Ok(fc)=>fc,
    //             Err(e)=> panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error =>{
    //             panic!("Problem opening, the file: {:?}", other_error);
    //         }
    //     }
    // };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if (error.kind() == ErrorKind::NotFound) {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });

    // let f2 = File::open("hello2.txt").expect("Could not open hello2.txt");

    // Propagating errors
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_fileV2() -> Result<String, io::Error> {
    // The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6.
    // If the value of the Result is an Ok,
    // the value inside the Ok will get returned from this expression, and the program will continue.
    //  If the value is an Err, the Err will be returned from the whole function as if we had used the
    // return keyword so the error value gets propagated to the calling code
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_fileV3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_fileV4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Option is also compatible with "?"
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
