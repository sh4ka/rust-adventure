mod models {
    pub mod player;
    pub mod object;
    pub mod character;
    pub mod enemy;
}
mod parsexec;
mod character_creation;

use std::io::{self, Write};
use crate::models::player::Player;
use crate::models::enemy::{Enemy, get_enemy, get_enemies_by_difficulty, get_enemies_by_level_range, get_enemies_by_location};
use crate::parsexec::{parse_command, execute_command};

struct Game {
    player: Player
}

impl Game {
    fn new(player: Player) -> Self {
        Game { player }
    }
}

fn main() {
    println!("Bienvenido a Aventura en Woodspring");
    println!("-----------------------------------\n");

    let (characters, initial_inventory) = character_creation::create_character_party();

    let mut player = Player::new(characters);
    player.inventory = initial_inventory;
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
        
        if input == "salir" {
            break;
        }

        let command = parse_command(&input);
        let response = execute_command(&mut game.player, command);
        if !response.is_empty() {
            println!("{}", response);
        }
    }
}
