extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    for x in 0..3 {
        println!("{} Hello World!", x);
    }

    let actual = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        println!("Guess a number: ");
        io::stdin().read_line(&mut guess).ok().expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&actual) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
