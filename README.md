# NUMBERS TELEPATHY GAME

> Hi friend, this is a fun little game where you try to **read the computer's mind** by guessing its secret number!

---

## How to Play

1. **Start the game**
   - The computer picks a secret whole number between **1 and 365** (yes, like guessing a birthday!).

2. **Make a guess**
   - Type in a number and press **Enter**.

3. **Get hints**
   - If your guess is **too low**, the computer will say:
     > `"Too small bro, ask for more."`
   - If your guess is **too high**, it'll say:
     > `"Jokes on you, that's too much bro, don't be greedy."`

4. **Keep guessing**
   - Use the hints to narrow it down within range.

5. **Win!**
   - When you guess it right, the computer celebrates:
     > `"God of Telepathy, we hail thee! [number] is RIGHT!"`
   - Then you can choose to **Restart** (type `R`) or **Exit** (type `E`).

---

## Fun Features

- **Playful personality** – The computer talks like a buddy, not a robot.
- **Forgiving** – Type letters by mistake? No problem! Just try again.
- **Big range** – Guessing from 1–365 is trickier and more fun than 1–100.
- **Quick to play** – Perfect for a short break or a fun challenge with friends.
- **Restart option** – Keep playing without relaunching the game!

---

## Example Round

```
WELCOME TO OBOT'S NUMBERS TELEPATHY GAME!
I'm Thinking Of A WHOLE NUMBER BETWEEN 1 AND 365
You Can Now Make Your Wildest Guesses

150
150's too small bro, ask for more.

300
Jokes on you, 300's too much bro, don't be greedy.

225
225's too small bro, ask for more.

275
God of Telepathy, we hail thee!
275 is RIGHT!

Type capital R to restart OR capital E to end the game
R
Restarting the game...
```

---

## Great For...

- **Complete beginners** – No gaming experience needed!
- **Quick fun** – Play during a coffee break
- **Friends & family** – See who has the best "telepathic" skills
- **Learning about computers** – See how guessing games work behind the scenes
- **Learning Rust** – See modular code organization in action!

---

## Tip

Play it like "Hot & Cold" with a computer friend. The smartest move: start in the middle (around 180) and use the hints to zero in on the secret number!

**Ready to test your mind-reading abilities?**

---

## Project Structure (Modular Design)

```
numbers_telepathy_game/
├── Cargo.toml
├── Cargo.lock (automatically generated)
├── README.md
└── src/
    ├── main.rs
    ├── game.rs
    ├── input.rs
    └── restart.rs
```

### Module Breakdown

| File | Responsibility |
|------|----------------|
| `main.rs` | Entry point - starts the game |
| `game.rs` | Core game logic, secret number, win/loss conditions |
| `input.rs` | User input handling, parsing, error messages |
| `restart.rs` | Game restart and exit flow control |

---

## 📁 Files Needed

**Cargo.toml:**

```toml
[package]
name = "numbers_telepathy_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

**src/main.rs:**

```rust
mod game;
mod input;
mod restart;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
```

**src/game.rs:**

```rust
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
```

**src/input.rs:**

```rust
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
    2. Just type numbers only, be honest to yourself man.\n 
    3. Confirm you've actually typed something.\n 
    4. Remove negative signs if you have any.");
}
```

**src/restart.rs:**

```rust
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
```

---

## How to Run

1. Install Rust (if you haven't already)
2. Create the project folder with the files above
3. Open terminal in the project folder
4. Run:
   ```bash
   cargo run
   ```
5. Start guessing!

---

## OR: Quick Installation

**Clone the repository:**

```bash
git clone https://github.com/oboobotenefiok/numbers_telepathy_game.git
```

**Enter the game directory:**

```bash
cd numbers_telepathy_game
```

**Run the game:**

```bash
cargo run
```

When you run the game for the first time, Cargo will automatically create a `Cargo.lock` file.

---

## What Makes This Special?

This Rust project demonstrates:

- **Modular architecture** – Clean separation of concerns across 4 modules
- **Random number generation** – Using the `rand` crate
- **User input handling** – Robust parsing with error recovery
- **Game logic** – Loops, conditionals, and match statements
- **Flow control** – Restart/exit functionality without recursion issues
- **Clean code practices** – Single responsibility principle in action

---

## Learning Rust Through This Game

| Concept | Where to Find It |
|---------|-----------------|
| Structs & impl blocks | `game.rs` — `Game` struct |
| Modules & visibility | `main.rs` — `mod` declarations |
| Error handling | `input.rs` — `match` with `parse()` |
| Crates & dependencies | `Cargo.toml` — `rand` dependency |
| Loops & control flow | `game.rs` — `loop` and `match` |
| Enums | `std::cmp::Ordering` |

---

Good Luck & Good Playing :-)

**Obot Obo** — Developer
