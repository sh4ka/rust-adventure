mod models {
    pub mod player;
    pub mod object;
    pub mod character;
}
mod parsexec;

use std::io::{self, Write};
use crate::models::player::Player;
use crate::models::character::{Character, Class, EquipmentType};
use crate::models::object::Item;
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
    let mut initial_inventory = Vec::new();
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
        let input = input.trim();

        let (character, items) = match input {
            "1" => {
                let espada = Item::new_equipment("espada_inicial", "una espada de hierro", true, EquipmentType::Weapon, 0);
                let armadura = Item::new_equipment("armadura_inicial", "una armadura de cuero", true, EquipmentType::Armor, 1);
                let mut character = Character::new(Class::Fighter);
                character.equip(espada.to_equipment().unwrap());
                character.equip(armadura.to_equipment().unwrap());
                (character, vec![espada, armadura])
            },
            "2" => {
                let hacha = Item::new_equipment("hacha_inicial", "un hacha de batalla", true, EquipmentType::Weapon, 1);
                let escudo = Item::new_equipment("escudo_inicial", "un escudo de madera", true, EquipmentType::Shield, 1);
                let mut character = Character::new(Class::Cleric);
                character.equip(hacha.to_equipment().unwrap());
                character.equip(escudo.to_equipment().unwrap());
                (character, vec![hacha, escudo])
            },
            "3" => {
                let daga = Item::new_equipment("daga_inicial", "una daga ligera", true, EquipmentType::Weapon, -1);
                let mut character = Character::new(Class::Rogue);
                character.equip(daga.to_equipment().unwrap());
                (character, vec![daga])
            },
            "4" => {
                let daga = Item::new_equipment("daga_inicial", "una daga ligera", true, EquipmentType::Weapon, -1);
                let mut character = Character::new(Class::Wizard);
                character.equip(daga.to_equipment().unwrap());
                (character, vec![daga])
            },
            "5" => {
                let hacha = Item::new_equipment("hacha_inicial", "un hacha de batalla", true, EquipmentType::Weapon, 1);
                let mut character = Character::new(Class::Barbarian);
                character.equip(hacha.to_equipment().unwrap());
                (character, vec![hacha])
            },
            "6" => {
                let espada = Item::new_equipment("espada_inicial", "una espada de hierro", true, EquipmentType::Weapon, 0);
                let mut character = Character::new(Class::Elf);
                character.equip(espada.to_equipment().unwrap());
                (character, vec![espada])
            },
            "7" => {
                let hacha = Item::new_equipment("hacha_inicial", "un hacha de batalla", true, EquipmentType::Weapon, 1);
                let armadura = Item::new_equipment("armadura_inicial", "una armadura de cuero", true, EquipmentType::Armor, 1);
                let mut character = Character::new(Class::Dwarf);
                character.equip(hacha.to_equipment().unwrap());
                character.equip(armadura.to_equipment().unwrap());
                (character, vec![hacha, armadura])
            },
            "8" => {
                let daga = Item::new_equipment("daga_inicial", "una daga ligera", true, EquipmentType::Weapon, -1);
                let mut character = Character::new(Class::Halfling);
                character.equip(daga.to_equipment().unwrap());
                (character, vec![daga])
            },
            _ => {
                println!("Opción no válida, se creará un guerrero por defecto.");
                let espada = Item::new_equipment("espada_inicial", "una espada de hierro", true, EquipmentType::Weapon, 0);
                let armadura = Item::new_equipment("armadura_inicial", "una armadura de cuero", true, EquipmentType::Armor, 1);
                let mut character = Character::new(Class::Fighter);
                character.equip(espada.to_equipment().unwrap());
                character.equip(armadura.to_equipment().unwrap());
                (character, vec![espada, armadura])
            }
        };
        characters.push(character);
        initial_inventory.extend(items);
    }

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
