mod models {
    pub mod player;
    pub mod object;
    pub mod character;
    pub mod enemy;
}
mod parsexec;

use std::io::{self, Write};
use crate::models::player::Player;
use crate::models::character::{Character, Class, EquipmentType, WeaponType, ArmorType};
use crate::models::object::Item;
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

    println!("Crea tu grupo de aventureros:");
    let mut characters = Vec::new();
    let mut initial_inventory = Vec::new();
    for i in 1..=4 {
        println!("Aventurero {}:", i);
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

        let (mut character, items) = match input {
            "1" => {
                let sword_warrior = Item::new_equipment("espada_guerrero", "una espada corta de hierro", true, EquipmentType::Weapon(WeaponType::Medium));
                let shield_warrior = Item::new_equipment("escudo_guerrero", "una escudo de hierro viejo", true, EquipmentType::Shield);
                let armor_warrior = Item::new_equipment("armadura_guerrero", "una armadura ligera de escamas", true, EquipmentType::Armor(ArmorType::Light));
                let mut character = Character::new(Class::Fighter);
                character.equip(sword_warrior.to_equipment().unwrap());
                character.equip(shield_warrior.to_equipment().unwrap());
                character.equip(armor_warrior.to_equipment().unwrap());
                (character, vec![sword_warrior, shield_warrior, armor_warrior])
            },
            "2" => {
                let mace_cleric = Item::new_equipment("maza_clerigo", "una maza", true, EquipmentType::Weapon(WeaponType::Medium));
                let shield_cleric = Item::new_equipment("escudo_clerigo", "una escudo de madera", true, EquipmentType::Shield);
                let armor_cleric = Item::new_equipment("armadura_clerigo", "una armadura ligera de malla", true, EquipmentType::Armor(ArmorType::Light));
                let mut character = Character::new(Class::Cleric);
                character.equip(mace_cleric.to_equipment().unwrap());
                character.equip(shield_cleric.to_equipment().unwrap());
                character.equip(armor_cleric.to_equipment().unwrap());
                (character, vec![mace_cleric, shield_cleric, armor_cleric])
            },
            "3" => {
                let rope = Item::new_equipment("cuerda_inicial", "una cuerda de cáñamo en buen estado", false, EquipmentType::Basic);
                let picks = Item::new_equipment("ganzuas", "un juego de ganzúas básico", false, EquipmentType::Basic);
                let armor_rogue = Item::new_equipment("armadura_picaro", "una armadura ligera de cuero", true, EquipmentType::Armor(ArmorType::Light));
                let dagger_rogue = Item::new_equipment("daga_picaro", "una daga", true, EquipmentType::Weapon(WeaponType::Light));
                let mut character = Character::new(Class::Rogue);
                character.equip(armor_rogue.to_equipment().unwrap());
                character.equip(dagger_rogue.to_equipment().unwrap());
                (character, vec![rope, picks, armor_rogue, dagger_rogue])
            },
            "4" => {
                let dagger_wizard = Item::new_equipment("daga_inicial", "una daga ligera", true, EquipmentType::Weapon(WeaponType::Light));
                let spellbook_wizard = Item::new_equipment("libro_de_hechizos", "un libro de hechizos", false, EquipmentType::Basic);
                let writing_implements = Item::new_equipment("elementos_de_escritura", "un conjunto de elementos de escritura", false, EquipmentType::Basic);
                let mut character = Character::new(Class::Wizard);
                character.equip(dagger_wizard.to_equipment().unwrap());
                (character, vec![dagger_wizard, spellbook_wizard, writing_implements])
            },
            "5" => {
                let hacha_barbarian = Item::new_equipment("hacha_barbaro", "un hacha de guerra", true, EquipmentType::Weapon(WeaponType::Medium));
                let shield_barbarian = Item::new_equipment("escudo_barbaro", "un escudo de madera reforzado", true, EquipmentType::Shield);
                let armor_barbarian = Item::new_equipment("armadura_barbaro", "una armadura ligera de pieles", true, EquipmentType::Armor(ArmorType::Light));
                let mut character = Character::new(Class::Barbarian);
                character.equip(hacha_barbarian.to_equipment().unwrap());
                character.equip(shield_barbarian.to_equipment().unwrap());
                character.equip(armor_barbarian.to_equipment().unwrap());
                (character, vec![hacha_barbarian, shield_barbarian, armor_barbarian])
            },
            "6" => {
                let espada_elf = Item::new_equipment("espada_elfo", "una espada larga de hierro", true, EquipmentType::Weapon(WeaponType::Medium));
                let armor_elf = Item::new_equipment("armadura_elfo", "una armadura de cuero", true, EquipmentType::Armor(ArmorType::Light));
                let bow_elf = Item::new_equipment("arco_elfo", "un arco", true, EquipmentType::Bow);
                let mut character = Character::new(Class::Elf);
                character.equip(espada_elf.to_equipment().unwrap());
                character.equip(armor_elf.to_equipment().unwrap());
                character.equip(bow_elf.to_equipment().unwrap());
                (character, vec![espada_elf, armor_elf, bow_elf])
            },
            "7" => {
                let hacha_dwarf = Item::new_equipment("hacha_enano", "un hacha de guerra", true, EquipmentType::Weapon(WeaponType::Medium));
                let shield_dwarf = Item::new_equipment("escudo_enano", "un pequeño escudo de madera reforzado", true, EquipmentType::Shield);
                let armadura_dwarf = Item::new_equipment("armadura_enano", "una armadura de cuero", true, EquipmentType::Armor(ArmorType::Light));
                let mut character = Character::new(Class::Dwarf);
                character.equip(hacha_dwarf.to_equipment().unwrap());
                character.equip(shield_dwarf.to_equipment().unwrap());
                character.equip(armadura_dwarf.to_equipment().unwrap());
                (character, vec![hacha_dwarf, shield_dwarf, armadura_dwarf])
            },
            "8" => {
                let snacks = Item::new_equipment("snacks", "un monton de snacks", false, EquipmentType::Basic);
                let sling_halfling = Item::new_equipment("honda_halfling", "una honda", true, EquipmentType::Bow);
                let daga_halfling = Item::new_equipment("daga_halfling", "una daga ligera", true, EquipmentType::Weapon(WeaponType::Light));
                let mut character = Character::new(Class::Halfling);
                character.equip(daga_halfling.to_equipment().unwrap());
                character.equip(sling_halfling.to_equipment().unwrap());
                (character, vec![daga_halfling, sling_halfling, snacks])
            },
            _ => {
                println!("Opción no válida, se creará un guerrero por defecto.");
                let espada = Item::new_equipment("espada_corta", "una espada corta de hierro", true, EquipmentType::Weapon(WeaponType::Medium));
                let shield = Item::new_equipment("escudo_hierro", "una escudo de hierro viejo", true, EquipmentType::Shield);
                let light_armor = Item::new_equipment("armadura_ligera_escamas", "una armadura ligera de escamas", true, EquipmentType::Armor(ArmorType::Light));
                let mut character = Character::new(Class::Fighter);
                character.equip(espada.to_equipment().unwrap());
                character.equip(shield.to_equipment().unwrap());
                character.equip(light_armor.to_equipment().unwrap());
                (character, vec![espada, shield, light_armor])
            }
        };

        print!("Escribe el nombre de tu personaje: ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        character.set_name(name.trim().to_string());

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
