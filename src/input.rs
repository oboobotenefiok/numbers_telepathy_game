use std::io;

pub fn get_player_guess() -> u32 {
    loop {
        let mut human_guess = String::new();
        
        io::stdin()
            .read_line(&mut human_guess)
            .expect("We need a number bro, jokes on you, lol :-)");
        
        match human_guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => display_input_error(),
        }
    }
}

fn display_input_error() {
    println!("That's not a WHOLE NUMBER, bro! Try again! \n 
    SUGGESTIONS: 1. Remove DECIMAL POINTS if you have any.\n 
    2. Just type numbers only, be honest to yourself man. Don't you know numbers? \n 
    3. Confirm you've actually typed something. \n 
    4. Remove negative signs if you have any.");
}