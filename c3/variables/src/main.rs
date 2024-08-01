// use std::io;

fn main() {
    let x = 5;

    println!("The value of x is: {x}");

    let x_plus_one = plus_one(x);
    println!("The value of x + 1 is: {x_plus_one}");
}

fn plus_one (x: i8) -> i8 {
    x + 1;
}
