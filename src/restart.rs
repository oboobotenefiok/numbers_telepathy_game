use std::io;

pub fn handle_restart() {
    println!("Type capital R to restart OR capital E to end the game");
    
    loop {
        let mut start_again = String::new();
        
        io::stdin()
            .read_line(&mut start_again)
            .expect("We need a decision bro, jokes on you, lol :-)");
        
        match start_again.trim() {
            "R" => {
                println!("Restarting the game...\n");
                crate::main();
                break;
            }
            "E" => {
                println!("Thanks for playing, telepathy master!");
                std::process::exit(0);
            }
            _ => {
                println!("You're rich in everything \n
                except the ability to follow simple instructions.\n
                Type capital R or capital E only, bro!");
                continue;
            }
        }
    }
}