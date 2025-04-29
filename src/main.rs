mod models {
    pub mod player;
    pub mod location;
    pub mod character;
}
mod parsexec;

use std::io::{self, Write};
use models::character::{parse_new_character, Character};
use models::player::Player;

const PROMPT: &str = "> ";

fn prompt() {
    print!("{}", PROMPT);
}

fn print_character_classes() {
    println!("Guerrero")
}

fn initial_characters() -> Vec<Character> {
    let mut initial_characters = Vec::new();
    println!("Elige la clase de personaje del primer aventurero.");
    let first_character = create_new_character();
    println!("Has elegido {:?}", first_character.class.to_string());
    initial_characters.push(first_character);
    println!("Elige la clase de personaje del segundo aventurero.");
    let second_character = create_new_character();
    println!("Has elegido {:?}", second_character.class.to_string());
    initial_characters.push(second_character);
    println!("Elige la clase de personaje del tercer aventurero.");
    let third_character = create_new_character();
    println!("Has elegido {:?}", third_character.class.to_string());
    initial_characters.push(third_character);
    println!("Elige la clase de personaje del cuarto aventurero.");
    let fourth_character = create_new_character();
    println!("Has elegido {:?}", fourth_character.class.to_string());
    initial_characters.push(fourth_character);
    println!("Que buen grupo!");
    initial_characters
}

fn create_new_character() -> Character {
    println!("Opciones:");
    print_character_classes();
    let input = capture_input();
    let new_character = parse_new_character(&input.to_string());
    if !new_character.is_ok() {
        create_new_character();
    }
    new_character.unwrap()
}

fn capture_input() -> String {
    prompt();
    io::stdout().flush().unwrap();
    let mut input = String::new(); // creamos una nueva variable mutable de tipo string
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    println!("Bienvenido a la \"Aventura en la Cueva\".");
    println!("Lo primero que debes hacer es crear un grupo de 4 aventureros.");
    let inital_characters = initial_characters();
    let mut player = Player::new(inital_characters);
    println!("Está muy oscuro.");
    println!("(Escribe \"salir\" para salir, \"ayuda\" para lista de comandos básicos.)");

    loop {
        let mut input = capture_input();

        let command = parsexec::parse_command(&input);
        if !parsexec::execute_command(&mut player, command) {
            break;
        }
    }
}
