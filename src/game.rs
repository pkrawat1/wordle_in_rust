use crate::word_list;
use std::io;

pub fn read_user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.to_lowercase().trim().to_string())
}

pub fn pretty_print(s: &str) {
    print!("{}", s.chars().next().unwrap());
    s.chars().skip(1).for_each(|c| print!("\t{}", c));
    println!("");
}

pub fn highlight_matched_chars(guess: &str, sol: &str) -> String {
    let green_square = '🟩';
    let black_square = '⬛';
    let yellow_square = '🟨';
    let sol_bytes = sol.as_bytes();
    let mut hint = String::new();
    for (i, b) in guess.bytes().enumerate() {
        let c = if b == sol_bytes[i] {
            green_square
        } else if sol_bytes.contains(&b) {
            yellow_square
        } else {
            black_square
        };
        hint.push(c);
    }
    hint
}

pub fn run() {
    println!("Let's Play Wordle");
    let words = word_list::WordList::new();
    let solution = words.pick_a_word();
    assert!(words.is_valid(&solution));
    let mut tries = 0;
    let max_tries = 6;
    println!("Enter a {0} letter word (max : {0} attempts) : ", max_tries);
    loop {
        println!("Please input your guess.");
        let guess = read_user_input().expect("Failed to read line");

        if !words.is_valid(&guess) {
            println!("'{}' is invalid guess", guess);
            continue;
        }

        tries += 1;
        let hint = highlight_matched_chars(&guess, &solution);
        pretty_print(&guess);
        pretty_print(&hint);

        if guess == solution {
            println!("Congrats \u{1F973}, you win!");
            return;
        }

        if tries < 6 {
          println!("{} more trie(s)", max_tries - tries);
        } else {
          break;
        }
    }

    println!("Oops \u{2639} game over, the answer was {}", solution);
}
