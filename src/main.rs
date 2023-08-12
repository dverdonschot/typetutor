use rust_typetutor::{char_training_set_lib_short, random_character_game};

fn main() {
    let characters = vec!['a', 'b', 'c', 'd', 'e']; // Example list of characters
    let training_set = char_training_set_lib_short(10, &characters);
    random_character_game(&training_set)
}
