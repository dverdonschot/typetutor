use colored::*;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;
use termios::{self, Termios};

#[derive(Debug)]
pub struct GameInput {
    typed_correct: String,
    random_character: char,
}

pub fn random_character_game(input: &BTreeMap<i32, GameInput>) -> BTreeMap<i32, GameInput> {
    let mut score: BTreeMap<i32, GameInput> = BTreeMap::new();
    for (character_id, game_status) in input.range(0..) {
        colorized_typed_results_string(&score, character_id);
        println!("\nType character: {}", &game_status.random_character);
        let typedkey = getkey().expect("No letter typed");

        if typedkey == game_status.random_character {
            println!("{}: {} is correct", character_id + 1, &typedkey);
            let inputs = GameInput {
                typed_correct: String::from("correct"),
                random_character: game_status.random_character,
            };
            score.insert(*character_id, inputs);
        } else {
            println!("{}: {} is incorrect", character_id + 1, &typedkey);
            let inputs = GameInput {
                typed_correct: String::from("incorrect"),
                random_character: game_status.random_character,
            };
            score.insert(*character_id, inputs);
        };
    }
    println!("{:?}", score);
    score
}

fn colorized_typed_results_string(input: &BTreeMap<i32, GameInput>, cycle: &i32) {
    for (_iteration, game_state) in input.range(0..*cycle) {
        if game_state.typed_correct == "correct" {
            print!("{}", game_state.random_character.to_string().green());
            io::stdout().flush().unwrap();
        } else {
            print!("{}", game_state.random_character.to_string().red());
        };
    }
}

pub fn char_training_set_lib_short(number: i32, characters: &[char]) -> BTreeMap<i32, GameInput> {
    let mut rng = thread_rng();
    let mut generated_char_training_set_lib: BTreeMap<i32, GameInput> = BTreeMap::new();

    for i in 0..number {
        let random_character2 = *characters.iter().choose(&mut rng).unwrap();

        let game_input_mut = GameInput {
            random_character: random_character2,
            typed_correct: String::from("unknown"),
        };
        generated_char_training_set_lib.insert(i, game_input_mut);
    }

    generated_char_training_set_lib
}

pub fn quote_training_set_lib_short(
    quote: String,
) -> BTreeMap<i32, GameInput> {

    let mut generated_quote_training_set_lib: BTreeMap<i32, GameInput> = BTreeMap::new();

    for (iteration, character) in quote.chars().enumerate() {
        let game_input_mut = GameInput {
            random_character: character,
            typed_correct: String::from("unknown"),
        };
        generated_quote_training_set_lib.insert(iteration as i32, game_input_mut);
    }

    generated_quote_training_set_lib
}

fn getkey() -> io::Result<char> {
    let stdin = io::stdin();
    let stdin_fd = stdin.as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd)?;
    let original_termios = termios.clone();
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(stdin_fd, termios::TCSANOW, &termios)?;

    let mut buf = [0u8; 1];
    let mut handle = stdin.lock();
    loop {
        match handle.read(&mut buf)? {
            0 => {
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            1 => break,
            _ => unreachable!(),
        }
    }

    termios::tcsetattr(stdin_fd, termios::TCSANOW, &original_termios)?;

    Ok(buf[0] as char)
}
