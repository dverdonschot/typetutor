
use rust_typetutor::{char_training_set_lib_short, quote_training_set_lib_short, random_character_game};
use menu_rs::{Menu, MenuOption};

fn main() {

    
    //let characters = vec!['a', 'b', 'c', 'd', 'e']; // Example list of characters
    //let training_set = char_training_set_lib_short(10, &characters);
    //random_character_game(&training_set)

    let menu = Menu::new( vec![
        MenuOption::new("Random Quotes", random_quotes),
        MenuOption::new("Random Characters", random_characters),
        MenuOption::new("Random Numpad", random_numpad),
        MenuOption::new("Quit", || std::process::exit(0)),
    ]);

    menu.show();

}

fn random_characters() {
    let characters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
    ]; // Example list of characters
    let training_set = char_training_set_lib_short(10, &characters);
    random_character_game(&training_set);
    main();
}

fn random_quotes() {
    let quote_training_set = quote_training_set_lib_short("Test String?".to_string());
    random_character_game(&quote_training_set);
    main();
}
fn random_numpad() {
    let letters = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '+', '-', '/', '*', '=', '.', ','
    ]; // Example list of characters
    let training_set = char_training_set_lib_short(10, &letters);
    random_character_game(&training_set);
    main();

}