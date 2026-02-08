use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("WELCOME TO OBOT'S NUMBERS TELEPATHY GAME!");
    let system_guess = rand::rng().random_range(1..=365);
    println!("You Can Now Make Your Wildest Guesses");
    loop {
        let mut human_guess = String::new();
        io::stdin().read_line(&mut human_guess).expect(
            "We need a
    number bro, jokes on you, lol :-)",
        );
        let human_guess: u32 = match human_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match human_guess.cmp(&system_guess) {
            Ordering::Less => println!(
                "{human_guess}'s too small bro,
    ask for more."
            ),
            Ordering::Greater => println!(
                "Jokes on you,
    {human_guess}'s too much bro, don't be greedy."
            ),
            Ordering::Equal => {
                println!(
                    "God of Telepathy, we hail thee! \n
    {system_guess} is RIGHT!"
                );
                break;
            }
        };
    }
}
