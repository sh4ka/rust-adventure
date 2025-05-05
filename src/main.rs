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

struct Game {
    player: Player
}

impl Game {
    fn new(player: Player) -> Self {
        Game { player }
    }

    fn show_status(&self) {
        self.player.show_status();
    }
}

fn main() {
    println!("Bienvenido a Aventura en Woodspring");
    println!("-----------------------------------\n");

    println!("Crea tu grupo de aventureros:");
    let mut characters = Vec::new();
    for i in 1..=4 {
        println!("\nAventurero {}:", i);
        println!("1. Guerrero");
        println!("2. Clérigo");
        println!("3. Pícaro");
        println!("4. Mago");
        println!("5. Bárbaro");
        println!("6. Elfo");
        println!("7. Enano");
        println!("8. Mediano");
        print!("Elige una clase (1-8): ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let class = input.trim();

        let character = match class {
            "1" => Character::new(Class::Fighter),
            "2" => Character::new(Class::Cleric),
            "3" => Character::new(Class::Rogue),
            "4" => Character::new(Class::Wizard),
            "5" => Character::new(Class::Barbarian),
            "6" => Character::new(Class::Elf),
            "7" => Character::new(Class::Dwarf),
            "8" => Character::new(Class::Halfling),
            _ => {
                println!("Opción no válida, se creará un guerrero por defecto.");
                Character::new(Class::Fighter)
            }
        };
        characters.push(character);
    }

    let player = Player::new(characters);
    let mut game = Game::new(player);
    println!("{}", game.player.execute_go(Some("pueblo")));

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
        let response = execute_command(command, &mut game.player);
        if !response.is_empty() {
            println!("{}", response);
        }

        if input == "salir" {
            break;
        }
    }
}
