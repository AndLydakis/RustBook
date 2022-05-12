use std::fmt::Result;
use std::io::Result as ioResult;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// fn function1() -> fmt::Result<()> {}

// fn function2() -> io::Result<()> {}
// fn function2() -> ioResult<()> {}
