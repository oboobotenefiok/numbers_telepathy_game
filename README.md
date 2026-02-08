```markdown
#  OBOT'S NUMBERS TELEPATHY GAME

> A fun little game where you try to **read the computer's mind** by guessing its secret number!

---

##  **How to Play**

1. **Start the game**
   - The computer picks a secret whole- number between **1 and 365** (yes, like guessing a birthday!).

2. **Make a guess**
   - Type in a number and press **Enter**.

3. **Get hints**
   - If your guess is **too low**, the computer will say:
     > `"Too small bro, ask for more."`
   - If your guess is **too high**, it'll say:
     > `"Jokes on you, that's too much bro, don't be greedy."`

4. **Keep guessing**
   - Use the hints to narrow it down.

5. **Win!**
   - When you guess it right, the computer celebrates:
     > `"God of Telepathy, we hail thee! [number] is RIGHT!"`

---

## s **Fun Features**

-  **Playful personality** – The computer talks like a buddy, not a robot.
- **Forgiving** – Type letters by mistake? No problem, just try again.
-  **Big range** – Guessing from 1–365 is trickier and more fun than 1–100!
- **Quick to play** – Perfect for a short break or a fun challenge with friends.

---

##  **Example Round**

```

WELCOME TO OBOT'S NUMBERS TELEPATHY GAME!
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

```

---

##  **Great For...**

- **Complete beginners** – No gaming experience needed!
- **Quick fun** – Play during a coffee break 
- **Friends & family** – See who has the best "telepathic" skills
- **Learning about computers** – See how guessing games work behind the scenes

---

##  **Tip**
Think of it like playing "Hot & Cold" with a computer friend. Start in the middle (around 180) and use the hints to zero in on the secret number!

**Ready to test your mind-reading abilities?** 

---

##  **Project Structure**
```

NUMBERS TELEPATHY GAME/
├── Cargo.toml
└── src/
└── main.rs
|__ README.md

```

---

## 📁 **Files Needed**

**Cargo.toml:**
```toml
[package]
name = "obot_telepathy_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

src/main.rs:

```rust
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
}
```

---

 How to Run

1. Install Rust (if you haven't already)
2. Create the project folder with the files above
3. Open terminal in the project folder
4. Run:
   ```bash
   cargo run
   ```
5. Start guessing!

---

What is This?

This is a Rust programming project that demonstrates:

· Random number generation
· User input handling
· Game logic with loops and conditions
· Fun console interactions

Perfect for learning Rust or just having some quick, entertaining fun!

```