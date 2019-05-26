// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!
macro_rules! my_macro {
    ($val:expr) => {
        {
            let mut t = String::from("Hello ");
            t.push_str($val);
            t
        }
    }
}

macro_rules! panic {
    ($val:expr) => {
        println!($val);
    }
}


fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
