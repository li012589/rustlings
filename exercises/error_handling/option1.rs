// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)
fn my_print<T>(term: Option<T>) where T: std::fmt::Debug {
    match term {
        Some(thing) => {
            println!("The last item in the list is {:?}", thing);
        },
        None => {
            println!("The last item in the list is None");
        },
    };
}
fn main() {
    let mut list = vec![3];

    let last = list.pop();
    my_print::<i32>(last);
    //println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop();
    my_print::<i32>(second_to_last);
    //println!("The second-to-last item in the list is {:?}", second_to_last);
}

























// Try using a `match` statement where the arms are `Some(thing)` and `None`.
// Or set a default value to print out if you get `None` by using the
// function `unwrap_or`.
// Or use an `if let` statement on the result of `pop()` to both destructure
// a `Some` value and only print out something if we have a value!
