use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // guessing();
    loop {
        println!("Please enter the N to get Nth pibonacci number.");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");
        let index: u32 = match index.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter number.not string");
                continue;
            },
        };
        let mut counter = 1;
        let mut i_2: u32 = 1;
        let mut i_1: u32 = 1;
        let mut i: u32 = 0;
        let result = loop {
            counter += 1;
            i = i_1 + i_2;
            i_1 = i_2;
            i_2 = i;
            if counter == index - 1 {
                break i;
            }
        };

        println!("The nth number is {}!", result);
    }
}

fn guessing() {
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
