use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("WELCOME TO OBOT'S NUMBERS TELEPATHY GAME! \n
    I'm Thinking Of A WHOLE NUMBER BETWEEN 1 AND 365");
    
    let system_guess = rand::thread_rng().gen_range(1..=365);
    println!("You Can Now Make Your Wildest Guesses");
    
    loop {
        let mut human_guess = String::new();
        
        io::stdin().read_line(&mut human_guess).expect(
            "We need a number bro, jokes on you, lol :-)",
        );
        
        let human_guess: u32 = match human_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a WHOLE NUMBER, bro! Try again! \n SUGGESTIONS: 1. Remove DECIMAL POINTS if you have any.\n 2. Just type numbers only, be honest to yourself man. Don't you know numbers?");
                continue;
            }
        };
        
        if human_guess >=1 && human_guess <=365 {  match human_guess.cmp(&system_guess) {
            Ordering::Less => println!(
                "{human_guess}'s too small bro, ask for more."
            ),
            Ordering::Greater => println!(
                "Jokes on you, {human_guess}'s too much bro, don't be greedy."
            ),
            Ordering::Equal => {
                println!(
                    "God of Telepathy, we hail thee! \n
                    {system_guess} is RIGHT!"
                );
                restart();
                return; 
            }
        };
        }
        else {
            println!("Please type a number within the range of 1 and 365");
            continue;
        }
    }
}

fn restart() {
    println!("Type capital R to restart OR capital E to end the game");
    
    loop {
        let mut start_again = String::new();
        
        io::stdin().read_line(&mut start_again).expect(
            "We need a decision bro, jokes on you, lol :-)",
        );
        
        match start_again.trim() {
            "R" => {
                println!("Restarting the game...\n");
                main();
                break;
            }
            "E" => {
                println!("Thanks for playing, telepathy master!");
                break;
            }
            _ => {
                println!("You're rich in everything \n
                except the ability to follow
                simple instructions.\n
                Type capital R or capital E only, bro!");
                continue;
            }
        }
    }
}

