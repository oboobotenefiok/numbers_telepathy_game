use rand::Rng;
use std::cmp::Ordering;
use crate::input::get_player_guess;
use crate::restart::handle_restart;

pub struct Game {
    secret_number: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            secret_number: rand::thread_rng().gen_range(1..=365),
        }
    }
    
    pub fn start(&mut self) {
        self.display_welcome();
        self.play_loop();
    }
    
    fn display_welcome(&self) {
        println!("WELCOME TO OBOT'S NUMBERS TELEPATHY GAME! \n
        I'm Thinking Of A WHOLE NUMBER BETWEEN 1 AND 365");
        println!("You Can Now Make Your Wildest Guesses");
    }
    
    fn play_loop(&mut self) {
        loop {
            let human_guess = get_player_guess();
            
            if !self.is_valid_guess(human_guess) {
                println!("Please type a number within the range of 1 and 365");
                continue;
            }
            
            match self.check_guess(human_guess) {
                Ordering::Less => println!("{human_guess}'s too small bro, ask for more."),
                Ordering::Greater => println!("Jokes on you, {human_guess}'s too much bro, don't be greedy."),
                Ordering::Equal => {
                    self.handle_win();
                    return;
                }
            }
        }
    }
    
    fn is_valid_guess(&self, guess: u32) -> bool {
        guess >= 1 && guess <= 365
    }
    
    fn check_guess(&self, guess: u32) -> Ordering {
        guess.cmp(&self.secret_number)
    }
    
    fn handle_win(&self) {
        println!(
            "God of Telepathy, we hail thee! \n
            {} is RIGHT!",
            self.secret_number
        );
        handle_restart();
    }
}