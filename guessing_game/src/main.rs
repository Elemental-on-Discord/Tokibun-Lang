use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let rnd_num = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number!");

    loop {

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");
    
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, dingus!");
                continue;
            },
        };

        match guess.cmp(&rnd_num) {
            Ordering::Less => println!("Your number is too small!\nTry again!"),
            Ordering::Equal => {
                println!("Your number is the same! Good Job!\nPlay Again?");
                break;
            },
            Ordering::Greater => println!("Your number is too big!\nTry again!"),
        }

    }
}
