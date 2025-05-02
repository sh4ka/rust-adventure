mod models {
    pub mod player;
    pub mod object;
    pub mod character;
}
mod parsexec;

use std::io::{self, Write};
use crate::models::player::Player;
use crate::models::character::{Character, Class};
use crate::parsexec::{parse_command, execute_command};

fn main() {
    println!("Bienvenido a Aventura en Woodspring");
    println!("Crea tu grupo de aventureros (4 personajes):");
    
    let mut characters = Vec::new();
    for i in 1..=4 {
        println!("\nPersonaje {}", i);
        println!("Elige una clase:");
        println!("1. Guerrero");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        
        let character = match input.trim() {
            "1" => Character::new(Class::Fighter),
            _ => Character::new(Class::Fighter)
        };
        characters.push(character);
    }
    
    let mut player = Player::new(characters);
    player.execute_go(Some("pueblo".to_string()));
    
    println!("\nEscribe 'ayuda' para ver los comandos disponibles.");
    
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        
        let command = parse_command(&input);
        execute_command(command, &mut player);
    }
}
