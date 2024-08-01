fn main() {
    let mut s1 = String::from("hello");
    
    take_ownership(s1);
    let x = 5;
    let s2 = s1.leak();

    // println!("{s1}");

    println!("{s2}");
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}
