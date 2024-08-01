use std::io::{self, Write};

fn main() {

    print!("Tap what you want: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    match input {
        "term" => termometer(),
        "fib" => fibonacci(),
        "xmas" => christmas_carol(),
        _ => println!("Invalid input!")
    }
}

fn termometer() {
    print!("What you want to convert? (C or F): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    print!("Enter the temperature: ");
    io::stdout().flush().unwrap();
    loop {
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp).expect("Failed to read line");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        if input == "C".to_lowercase() {
            let f = temp * 9.0 / 5.0 + 32.0;
            println!("{}°C is {}°F", temp, f);
            break;
        } else if input == "F".to_lowercase() {
            let c = (temp - 32.0) * 5.0 / 9.0;
            println!("{}°F is {}°C", temp, c);
            break;
        } else {
            println!("Please enter C or F!");
            break;
        }
    }
}

fn fibonacci() {
    print!("Enter the number of terms: ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 0;
    while count < n {
        print!("{n1} ");
        let n_temp = n1 + n2;
        n1 = n2;
        n2 = n_temp;
        count += 1;
    }
    println!();
}

fn christmas_carol() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            println!("{} {}", gifts[j], if i > 0 && j == 1 { "and" } else { "" });
        }
        println!();
    }
}