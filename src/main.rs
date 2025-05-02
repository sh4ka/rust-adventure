mod models {
    pub mod player;
    pub mod object;
    pub mod character;
}
mod parsexec;

use std::io::{self, Write};
use models::character::{parse_new_character, Character};
use models::player::Player;
use crate::models::object::{find_object_by_tag, OBJECTS};

const PROMPT: &str = "> ";

fn prompt() {
    print!("{}", PROMPT);
}

fn print_character_classes() {
    println!("Guerrero")
}

fn initial_characters() -> Vec<Character> {
    let mut initial_characters = Vec::new();
    println!("Decide quién irá en primer lugar.");
    let first_character = create_new_character();
    initial_characters.push(first_character);
    println!("Quién irá en segundo lugar?");
    let second_character = create_new_character();
    initial_characters.push(second_character);
    println!("Quién irá en tercer lugar?");
    let third_character = create_new_character();
    initial_characters.push(third_character);
    println!("Quién irá en cuarto lugar?");
    let fourth_character = create_new_character();
    initial_characters.push(fourth_character);
    println!("Qué buen grupo de aventureros!");
    initial_characters
}

fn create_new_character() -> Character {
    println!("Opciones:");
    print_character_classes();
    let input = capture_input();
    let new_character = parse_new_character(input);
    if !new_character.is_ok() {
        return create_new_character();
    }
    new_character.unwrap()
}

fn capture_input() -> String {
    prompt();
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {    
    println!("Bienvenido a \"Aventura en Woodspring\".");
    println!("Lo primero que debes hacer es crear un grupo de 4 aventureros.");
    let inital_characters = initial_characters();
    let mut player = Player::new(inital_characters);
    println!("(Escribe \"salir\" para salir, \"ayuda\" para lista de comandos básicos.)");
    let initial_location = find_object_by_tag("pueblo");
    player.execute_go(initial_location);
    
    let mut turn = 0;
    
    loop {
        let input = capture_input();
        let game_command = parsexec::parse_command(&input);

        if game_command.is_some() {
            if !parsexec::execute_command(&mut player, &game_command.unwrap()) {
                continue;
            }
            turn += 1; // command executed ok
        }                
    }
}
