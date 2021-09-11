use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's guess the number!");

    loop {
        let rand_number = rand::thread_rng().gen_range(1..=10);

        println!("Enter the number you thought");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        println!("You guessed {}", number);

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not number!");
                continue;
            },
        };

        match number.cmp(&rand_number) {
            Ordering::Equal => {
                println!("Perfect!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
        }

        println!("The correct answer is {}", rand_number);
    }
}
