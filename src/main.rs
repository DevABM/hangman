use std::io::{self, Write};

fn main() {
    let secret_word = "hangman";
    let mut guessed_letters = vec!['_'; secret_word.len()];
    let mut attempts = 6;

    println!("Welcome to Hangman!");

    loop {
        println!("Secret word: {}", guessed_letters.iter().collect::<String>());
        println!("Attempts left: {}", attempts);
        print!("Guess a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input.");
        let guess = guess.trim().chars().next().unwrap();

        let mut found_match = false;
        for (index, letter) in secret_word.chars().enumerate() {
            if letter == guess {
                guessed_letters[index] = letter;
                found_match = true;
            }
        }

        if !found_match {
            attempts -= 1;
            println!("Incorrect guess.");
        }

        if guessed_letters.iter().all(|&c| c != '_') {
            println!("Congratulations, you guessed the word: {}", secret_word);
            break;
        }

        if attempts == 0 {
            println!("You've run out of attempts. The word was: {}", secret_word);
            break;
        }
    }
}
