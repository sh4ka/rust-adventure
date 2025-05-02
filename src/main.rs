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
    println!("-----------------------------------\n");

    println!("Crea tu grupo de aventureros:");
    let mut characters = Vec::new();
    for i in 1..=4 {
        println!("\nAventurero {}:", i);
        println!("1. Guerrero");
        println!("2. Mago");
        println!("3. Ladrón");
        println!("4. Clérigo");
        print!("Elige una clase (1-4): ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let _class = input.trim();

        // Por ahora, todos son guerreros
        characters.push(Character::new(Class::Fighter));
    }

    let mut player = Player::new(characters);
    println!("{}", player.execute_go(Some("pueblo")));

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }

        let command = parse_command(&input);
        let response = execute_command(command, &mut player);
        if !response.is_empty() {
            println!("{}", response);
        }

        if input == "salir" {
            break;
        }
    }
}
