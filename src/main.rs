use rand::seq::SliceRandom;
use colored::*;
use std::io;
use std::io::Write;

mod utility;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    
    loop {

        clearscreen::clear().unwrap();
        println!("Welcome to hangcrab!\nAn 'hangman' game written by xflea in Rust.");
        println!("Words are provided by REST API at https://random-word-api.herokuapp.com/word");
        println!("\nPlease select the language:");
        println!("it - Italiano");
        println!("es - Español");
        println!("de - Deutsch");
        println!("Other input will extract an English word.");
        println!("\nIf you want to quit the game, type 'q'.");

        let mut language = String::new();
        let mut language_resp = String::from("https://random-word-api.herokuapp.com/word");
        print!("\n> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut language).unwrap();

        if language.trim() == "it" || language.trim() == "es" || language.trim() == "de" {
            let mut param = String::from("?lang=");    
            param.push_str(&language.trim());
            language_resp.push_str(&param);
        }
        else if language.trim() == "q" {
            break;
        }

        let mut streak: i32 = 0;

        loop {

            clearscreen::clear().unwrap();
            println!("Retriving the word from server... Please wait...");

            let resp: String = reqwest::Client::new().get(&language_resp).send().await?.text().await?;
            let word = utility::purify_word(resp);
            let mut string_guess;
            let mut guess = String::new();
            let mut errors: i8 = 5;
            let mut letters_to_guess: Vec<char> = Vec::new();
            let mut tries: Vec<char> = Vec::new();

            // add the letter to the vector
            for single in word.chars() {
                if !letters_to_guess.contains(&single) {
                    letters_to_guess.push(single);
                }
            }

            // extracting the hint
            let sample: Vec<_> = letters_to_guess.choose_multiple(&mut rand::thread_rng(), 1).collect();
            let hint = *sample[0];
            tries.push(hint);
            letters_to_guess.retain(|&x| x != hint);

            string_guess = "Choose wisely".bold().italic().to_string();

            // game loop starts
            loop {

                clearscreen::clear().unwrap();

                println!("Errors available: {} - {}", errors, string_guess);
                println!("Current streak - {}", streak);
                println!("Your tries - {:?}\n\n", tries);

                utility::print_to_guess(&word, &mut tries);

                // read the guess
                print!("\n> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut guess).unwrap();

                if guess.trim().to_ascii_lowercase().chars().count() == 1 {

                    let first_char = guess.chars().next().unwrap();

                    if tries.contains(&first_char) {
                        string_guess = "You already typed this entry, please select another...".bold().italic().to_string();
                    }
                    else {

                        tries.push(first_char);

                        if letters_to_guess.contains(&first_char) {
                            letters_to_guess.retain(|&x| x != first_char);
                            string_guess = "CORRECT!".green().bold().italic().to_string();
                        }
                        else {
                            errors = errors - 1;
                            string_guess = "Wrong...".red().bold().italic().to_string();
                        }

                        if errors < 0 {
                            println!("\nYou lost... better luck next time!");
                            streak = 0;
                            break;
                        }
                        else if letters_to_guess.is_empty() {
                            streak = streak + 1;
                            println!("\nYou won! The word was {}.\nYou're awsome!", word.green().bold().italic());
                            break;
                        }
                    }
                }
                else {
                    string_guess = "Please provide a valid input...".bold().italic().to_string();
                }
                guess = "".to_string();

            }

            let mut to_continue = String::new();
            print!("\nIf you wish to stop, type 'q', if you wish to continue, press any key!");
            print!("\n> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut to_continue).unwrap();

            if to_continue.trim() == "q" {
                break;
            }

        }

    }

    println!("Thanks for playing!");
    Ok(())

}
