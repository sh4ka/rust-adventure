use crate::models::character::Character;
use crate::models::object::{Location, Item, NPC, Passage, find_location, find_npc, find_item_in_location, find_passage, find_item, PASSAGES, Attitude, NPCTag};
use crate::models::character::{Equipment, EquipmentType, WeaponType, ArmorType, Class, parse_new_character};
use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::io::{self, Write};

use super::enemy;

pub trait InputReader {
    fn read_line(&mut self) -> String;
}

pub struct StdInputReader;

impl InputReader for StdInputReader {
    fn read_line(&mut self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }
}

pub struct TestInputReader {
    input: String,
}

impl TestInputReader {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl InputReader for TestInputReader {
    fn read_line(&mut self) -> String {
        self.input.clone()
    }
}

pub trait DiceRoller {
    fn roll_1d6(&mut self) -> u8;
}

pub struct RealDiceRoller;
impl DiceRoller for RealDiceRoller {
    fn roll_1d6(&mut self) -> u8 {
        rand::thread_rng().gen_range(1..=6)
    }
}

pub struct MockDiceRoller {
    pub value: u8,
}
impl DiceRoller for MockDiceRoller {
    fn roll_1d6(&mut self) -> u8 {
        self.value
    }
}

#[derive(Debug)]
pub struct Player {
    characters: Vec<Character>,
    pub current_location: Option<String>,  // Tag de la ubicación actual
    pub inventory: Vec<Item>,           // Items en el inventario
    pub search_attempts: HashMap<String, u32>,  // Sala -> número de intentos
    pub discovered_items: HashSet<String>,    // Tags de items descubiertos
    pub dropped_items: HashMap<String, String>, // tag -> ubicación donde se soltó
    pub picked_items: HashSet<String>,        // Tags de items recogidos
    pub discovered_locations: HashSet<String>, // Tags de localizaciones descubiertas
    pub defeated_npcs: HashSet<String>,        // Tags de NPCs derrotados
    pub current_combat_enemies: Option<u8>,    // Número de enemigos restantes en el combate actual
    pub encounters_won: u32,                   // Número de encuentros superados (excluyendo Vermin)
    pub leveled_up_last_time: Option<String>,  // Nombre del personaje que subió de nivel por última vez
}

impl Player {
    pub fn new(characters: Vec<Character>) -> Self {
        // Verificar nombres duplicados
        let mut names = HashSet::new();
        for character in &characters {
            if !names.insert(character.name.clone()) {
                panic!("No pueden haber personajes con el mismo nombre: {}", character.name);
            }
        }

        Self { 
            characters, 
            current_location: None,
            inventory: Vec::new(),
            search_attempts: HashMap::new(),
            discovered_items: HashSet::new(),
            dropped_items: HashMap::new(),
            picked_items: HashSet::new(),
            discovered_locations: HashSet::new(),
            defeated_npcs: HashSet::new(),
            current_combat_enemies: None,
            encounters_won: 0,
            leveled_up_last_time: None,
        }
    }

    fn set_current_location(&mut self, location_tag: Option<String>) {
        if let Some(tag) = &location_tag {
            if let Some(location) = find_location(tag) {
                // No podemos modificar location.content directamente porque es una referencia inmutable
                // En su lugar, podríamos mantener un registro de salas visitadas en el Player
                self.search_attempts.insert(tag.clone(), 0);
            }
        }
        self.current_location = location_tag;
    }

    pub fn execute_look(&self) {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                println!();
                println!("Estás en {}:", location.base.description);
                println!("- {}", location.base.long_description);

                // Obtener items visibles en la ubicación actual que no han sido recogidos ni soltados en otro lugar
                let visible_items: Vec<_> = location.content.items.iter()
                    .filter(|item| {
                        let is_visible = item.base.visible || self.discovered_items.contains(&item.base.tag);
                        let not_picked = !self.picked_items.contains(&item.base.tag);
                        let not_dropped_elsewhere = !self.dropped_items.contains_key(&item.base.tag);
                        is_visible && not_picked && not_dropped_elsewhere
                    })
                    .collect();

                // Obtener NPCs visibles en la ubicación actual
                let visible_npcs: Vec<_> = location.content.npcs.iter()
                    .filter_map(|npc_tag| find_npc(npc_tag))
                    .filter(|npc| npc.base.visible)
                    .collect();

                // Obtener items soltados en la ubicación actual
                let dropped_items: Vec<_> = self.dropped_items.iter()
                    .filter(|(_, loc_tag)| loc_tag.as_str() == location_tag.as_str())
                    .filter_map(|(item_tag, _)| find_item(item_tag))
                    .collect();

                // Mostrar items y npc en la ubicación solo si hay visibles
                let has_visible_items = !visible_items.is_empty() || !dropped_items.is_empty();
                let has_visible_npcs = !visible_npcs.is_empty();

                if has_visible_items || has_visible_npcs {
                    println!();
                    println!("Ves:");
                    for item in visible_items {
                        println!("- {}", item.base.description);
                    }
                    for item in dropped_items {
                        println!("- {}", item.base.description);
                    }
                    for npc in visible_npcs {
                        let attitude = match npc.attitude {
                            Attitude::Hostile => {
                                let remaining = if let Some(remaining) = self.current_combat_enemies {
                                    remaining
                                } else {
                                    npc.count
                                };
                                format!(" (hostil, nivel {}, x{})", npc.level, remaining)
                            },
                            Attitude::Neutral => " (neutral)".to_string(),
                            Attitude::Friendly => " (amistoso)".to_string(),
                        };
                        println!("- {}{}", npc.base.description, attitude);
                    }
                }

                // Mostrar las ubicaciones disponibles a las que el jugador puede ir
                println!();
                println!("Puedes ir a:");
                println!("{}", &self.show_available_locations());
            }
        }
    }

    pub fn execute_go(&mut self, location_tag: Option<&str>) -> String {
        // Verificar si hay enemigos hostiles en la ubicación actual
        if self.has_hostile_npcs() {
            return "¡No puedes huir! Hay enemigos hostiles aquí.".to_string();
        }

        match location_tag {
            Some(tag) => {
                if let Some(current_location) = self.current_location.as_ref() {
                    if let Some(location) = find_location(current_location) {
                        // Verificar si la ubicación destino está conectada
                        if location.connections.contains(&tag.to_string()) {
                            if let Some(destination) = find_location(tag) {
                                // Buscar pasaje en el mapa de pasajes
                                let passage = PASSAGES.values().find(|p| 
                                    (p.from == *current_location && p.to == tag) || 
                                    (p.from == tag && p.to == *current_location)
                                );

                                // Si encontramos un pasaje con requisitos especiales, verificarlos
                                if let Some(passage) = passage {
                                    // Verificar si el pasaje requiere un objeto
                                    if passage.requires_item {
                                        if let Some(item_tag) = &passage.item_tag {
                                            if !self.has_item(item_tag) {
                                                return format!("Necesitas un objeto concreto para pasar por {}.", passage.base.description);
                                            }
                                        }
                                    }

                                    // Verificar si el pasaje requiere una llave
                                    if passage.requires_key {
                                        if let Some(key_tag) = &passage.key_tag {
                                            if !self.has_item(key_tag) {
                                                return format!("Necesitas una llave para pasar por {}.", passage.base.description);
                                            }
                                        }
                                    }

                                    // Verificar si el pasaje tiene un acertijo
                                    if passage.has_riddle {
                                        if let (Some(riddle), Some(answer)) = (&passage.riddle, &passage.riddle_answer) {
                                            println!("\n{}", riddle);
                                            println!("Escribe tu respuesta (o 'cancelar' para volver):");

                                            let mut input = String::new();
                                            match std::io::stdin().read_line(&mut input) {
                                                Ok(_) => {
                                                    let input = input.trim().to_lowercase();
                                                    if input == "cancelar" {
                                                        return "Has decidido no intentar resolver el acertijo.".to_string();
                                                    }
                                                    if input != answer.to_lowercase() {
                                                        return "Respuesta incorrecta. La puerta permanece cerrada.".to_string();
                                                    }
                                                    // Solo si la respuesta es correcta, permitimos el paso
                                                }
                                                Err(_) => {
                                                    return "Error al leer la respuesta. La puerta permanece cerrada.".to_string();
                                                }
                                            }
                                        }
                                    }
                                }

                                // Si llegamos aquí, el jugador puede pasar
                                self.set_current_location(Some(tag.to_string()));
                                self.execute_look();
                                "".to_string()
                            } else {
                                return format!("No existe la ubicación '{}'.", tag);
                            }
                        } else {
                            return self.show_default_locations();
                        }
                    } else {
                        return format!("No existe la ubicación actual '{}'.", current_location);
                    }
                } else {
                    // Si no hay ubicación actual, permitir moverse a cualquier ubicación válida
                    if find_location(tag).is_some() {
                        self.set_current_location(Some(tag.to_string()));
                        self.execute_look();
                        "".to_string()
                    } else {
                        return format!("No existe la ubicación '{}'.", tag);
                    }
                }
            }
            None => {
                return self.show_default_locations();
            }
        }
    }

    fn show_default_locations(&self) -> String {
        let mut response = String::new();
        response.push_str("\nPuedes ir a:\n");
        response.push_str(&self.show_available_locations());
        response
    }

    pub fn execute_take(&mut self, item_tag: &str) -> bool {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                if let Some(item) = find_item_in_location(location, item_tag) {
                    if (item.base.visible || self.discovered_items.contains(&item.base.tag)) && !self.picked_items.contains(&item.base.tag) {
                        self.inventory.push(item.clone());
                        self.picked_items.insert(item.base.tag.clone());
                        println!("Has cogido {} y lo has añadido a tu inventario.", item.base.description);
                        return true;
                    }
                }
                println!("No hay ningún objeto con ese nombre aquí.");
                if !location.content.items.is_empty() {
                    println!("\nPuedes coger:");
                    for (i, item) in location.content.items.iter().enumerate() {
                        if (item.base.visible || self.discovered_items.contains(&item.base.tag)) && !self.picked_items.contains(&item.base.tag) {
                            if i == location.content.items.len() - 1 {
                                println!("- {} [{}].", item.base.description, item.base.tag);
                            } else {
                                println!("- {} [{}],", item.base.description, item.base.tag);
                            }
                        }
                    }
                } else {
                    println!("No hay nada que puedas coger aquí.");
                }
            }
        }
        false
    }

    pub fn execute_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Tu inventario está vacío.");
        } else {
            // Primero mostrar objetos no equipados
            let mut non_equipped_items: Vec<&Item> = Vec::new();

            for item in self.inventory.iter() {
                let is_equipped = if item.is_equipment {
                    let mut equipped = false;
                    for character in &self.characters {
                        if let Some(equipment) = item.to_equipment() {
                            match equipment.equipment_type {
                                EquipmentType::Basic => (),
                                EquipmentType::Weapon(_) => {
                                    if let Some(weapon) = &character.weapon {
                                        if weapon.name == item.base.description {
                                            equipped = true;
                                        }
                                    }
                                },
                                EquipmentType::Shield => {
                                    if let Some(shield) = &character.shield {
                                        if shield.name == item.base.description {
                                            equipped = true;
                                        }
                                    }
                                },
                                EquipmentType::Armor(_) => {
                                    if let Some(armor) = &character.armor {
                                        if armor.name == item.base.description {
                                            equipped = true;
                                        }
                                    }
                                },
                                EquipmentType::Bow => {
                                    if let Some(bow) = &character.bow {
                                        if bow.name == item.base.description {
                                            equipped = true;
                                        }
                                    }
                                },
                            }
                        }
                    }
                    equipped
                } else {
                    false
                };

                if !is_equipped {
                    non_equipped_items.push(item);
                }
            }

            if !non_equipped_items.is_empty() {
                println!("Objetos en el inventario:");
                for (i, item) in non_equipped_items.iter().enumerate() {
                    if i == non_equipped_items.len() - 1 {
                        println!("- {}.", item.base.description);
                    } else {
                        println!("- {},", item.base.description);
                    }
                }
            }

            // Luego mostrar objetos equipados
            let mut has_equipped = false;
            for character in &self.characters {
                if character.weapon.is_some() || character.shield.is_some() || 
                   character.armor.is_some() || character.bow.is_some() {
                    if !has_equipped {
                        println!("\nObjetos equipados:");
                        has_equipped = true;
                    }
                    if let Some(weapon) = &character.weapon {
                        println!("- {} (equipado por {} - {})", weapon.name, character.name, character.class);
                    }
                    if let Some(shield) = &character.shield {
                        println!("- {} (equipado por {} - {})", shield.name, character.name, character.class);
                    }
                    if let Some(armor) = &character.armor {
                        println!("- {} (equipado por {} - {})", armor.name, character.name, character.class);
                    }
                    if let Some(bow) = &character.bow {
                        println!("- {} (equipado por {} - {})", bow.name, character.name, character.class);
                    }
                }
            }

            if non_equipped_items.is_empty() && !has_equipped {
                println!("Tu inventario está vacío.");
            }
        }
    }

    pub fn execute_status(&self) {
        println!("Estado del grupo:");
        println!("=================");

        for character in &self.characters {
            println!(
                "- {} ({}, nivel {}): {} PV/{} PV",
                character.name,
                character.class,
                character.level,
                character.hit_points,
                character.max_hit_points
            );
        }

        println!("====================");
        println!("XP acumulados: {}/10", self.encounters_won);
    }

    pub fn has_item(&self, tag: &str) -> bool {
        self.inventory.iter().any(|item| item.base.tag == tag)
    }

    pub fn execute_search(&mut self) -> bool {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                // Obtener el número de intentos en esta sala
                let attempts = self.search_attempts.entry(location_tag.clone()).or_insert(0);
                *attempts += 1;

                // Calcular probabilidad de éxito
                let mut success_chance = 50; // Base 50%

                // Bonus por intentos previos (máximo 5 intentos)
                let attempts_bonus = (*attempts).min(5) * 5;
                success_chance += attempts_bonus as i32;

                // Bonus por tener antorcha
                if self.has_item("antorcha") {
                    success_chance += 20;
                }

                // Asegurar que la probabilidad esté entre 5% y 95%
                success_chance = success_chance.max(5).min(95);

                // Generar número aleatorio
                let mut rng = rand::thread_rng();
                let roll = rng.gen_range(1..=100);

                if roll <= success_chance {
                    let mut found_something = false;

                    // Buscar items ocultos en la sala
                    let hidden_items: Vec<_> = location.content.items.iter()
                        .filter(|item| {
                            !item.base.visible && !self.discovered_items.contains(&item.base.tag)
                        })
                        .collect();

                    // Buscar localizaciones ocultas en la sala
                    let hidden_locations: Vec<_> = location.connections.iter()
                        .filter_map(|tag| find_location(tag))
                        .filter(|loc| !loc.base.visible && !self.discovered_locations.contains(&loc.base.tag))
                        .collect();

                    for item in hidden_items {
                        println!("Has descubierto {}", item.base.description);
                        self.discovered_items.insert(item.base.tag.clone());
                        found_something = true;
                    }
                    for location in hidden_locations {
                        println!("Has descubierto {}", location.base.description);
                        self.discovered_locations.insert(location.base.tag.clone());
                        found_something = true;
                    }

                    // Reiniciar contador de intentos para esta sala
                    self.search_attempts.remove(location_tag);
                    return found_something;
                } else {
                    println!("No encuentras nada especial...");
                }
            }
        }
        false
    }

    pub fn execute_drop(&mut self, item_tag: &str) -> bool {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                if let Some(index) = self.inventory.iter().position(|item| item.base.tag == item_tag) {
                    // Remover el item del inventario
                    let item = self.inventory.remove(index);
                    // Añadir el item a la ubicación actual
                    self.picked_items.remove(&item.base.tag);
                    // Actualizar la ubicación del item
                    self.dropped_items.insert(item.base.tag.clone(), location_tag.clone());
                    println!("Has soltado {}.", item.base.description);
                    return true;
                } else {
                    println!("No tienes ese objeto en tu inventario.");
                }
            }
        }
        false
    }

    fn show_available_locations(&self) -> String {
        let mut response = String::new();
        let mut has_connections = false;
        if let Some(current_location) = &self.current_location {
            if let Some(location) = find_location(&current_location) {
                for connection in &location.connections {
                    if let Some(connected_location) = find_location(connection) {
                        if connected_location.base.visible || self.discovered_locations.contains(connection) {
                            response.push_str(&format!("- {} ({})\n", connected_location.base.description, connection));
                            has_connections = true;
                        }
                    }
                }
            }
        }
        if !has_connections {
            response.push_str("No hay salidas disponibles desde aquí.");
        }
        response
    }

    pub fn execute_attack(&mut self, target_tag: &str) {        

        let mut round = 0;
        let mut total_enemies_defeated = 0;

        // Obtener NPCs hostiles en la ubicación actual
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                let hostile_npcs: Vec<&NPC> = location.content.npcs.iter()
                    .filter_map(|npc_tag| find_npc(npc_tag))
                    .filter(|npc| npc.attitude == Attitude::Hostile && !self.defeated_npcs.contains(&npc.base.tag))
                    .collect();

                if hostile_npcs.is_empty() {
                    self.current_combat_enemies = None;
                    println!("No hay enemigos para atacar aquí.");
                    return;
                }

                // Si es una acción de combate (2-4), procesar la acción
                if target_tag == "2" {
                    // Huir
                    self.current_combat_enemies = None;
                    println!("Has huido del combate.");
                    return;
                } else if target_tag == "3" {
                    println!("Función de usar objetos aún no implementada.");
                    return;
                } else if target_tag == "4" {
                    return self.execute_status();
                }

                // Obtener el número de enemigos restantes
                let mut enemies_remaining = if target_tag == "continuar" || target_tag == "1" {
                    if let Some(remaining) = self.current_combat_enemies {
                        remaining
                    } else {
                        println!("No hay un combate en curso.");
                        return;
                    }
                } else {
                    hostile_npcs.iter().map(|npc| npc.count).sum()
                };

                // Bucle de combate que continúa hasta que no queden enemigos
                loop {
                    // Ejecutar una ronda de combate
                    let remaining = self.execute_combat_round(&hostile_npcs[0], enemies_remaining, &mut round);

                    // Calcular cuántos enemigos fueron derrotados en esta ronda
                    let enemies_defeated_this_round = enemies_remaining - remaining;
                    total_enemies_defeated += enemies_defeated_this_round;

                    println!("Enemigos derrotados en esta ronda: {}", enemies_defeated_this_round);
                    println!("Total de enemigos derrotados: {}", total_enemies_defeated);

                    // Actualizar el número de enemigos restantes
                    enemies_remaining = remaining;

                    // Si no quedan enemigos, terminar el combate
                    if remaining == 0 {
                        break;
                    }

                    // Si quedan enemigos, mostrar opciones y guardar el estado
                    self.current_combat_enemies = Some(remaining);
                    println!("¿Qué quieres hacer?");
                    println!("1. Continuar el combate");
                    println!("2. Huir");
                    println!("3. Usar un objeto");
                    println!("4. Ver estado detallado");

                    // Esperar la entrada del usuario
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            let choice = input.trim();
                            if choice == "2" {
                                // Huir
                                self.current_combat_enemies = None;
                                println!("Has huido del combate.");
                                return;
                            } else if choice == "3" {
                                println!("Función de usar objetos aún no implementada.");
                                // Continuar el combate después de usar un objeto
                            } else if choice == "4" {
                                self.execute_status();
                                // Continuar el combate después de ver el estado
                            } else if choice != "1" {
                                println!("Opción no válida. Continuando el combate...");
                            }
                        }
                        Err(_) => {
                            println!("Error al leer la entrada. Continuando el combate...");
                        }
                    }
                }

                // Combate terminado exitosamente
                self.current_combat_enemies = None;
                for npc in hostile_npcs {
                    self.defeated_npcs.insert(npc.base.tag.clone());
                    if !npc.has_tag(&NPCTag::Vermin) {
                        self.encounters_won += 1;
                    }
                }

                println!("¡Combate terminado! Has derrotado a {} enemigos en total.", total_enemies_defeated);

                // Verificar si se ha alcanzado el umbral de 10 encuentros
                if self.encounters_won >= 10 {
                    let mut input_reader = StdInputReader;
                    self.handle_level_up(&mut input_reader, &mut RealDiceRoller);
                }
            }
        }
    }

    fn execute_combat_round(&mut self, npc: &NPC, enemies_remaining: u8, round: &mut u8) -> u8 {
        *round += 1;
        if round == &1 {
            println!("¡Comienza el combate!");
        }

        // Fase de ataque de los aventureros
        println!("Ataque de los aventureros:");
        let mut enemies_defeated = 0;
        let enemies_outnumbered = self.characters.len() > enemies_remaining as usize;

        for character in &mut self.characters {
            if enemies_remaining == 0 || enemies_defeated == enemies_remaining {
                break;
            }

            match character.get_equipment_attack_bonus() {
                None => {
                    println!("{} ({}) no puede atacar porque no tiene un arma equipada.\n",
                             character.name, character.class);
                    continue;
                },
                Some(equipment_bonus) => {
                    let attack_roll = rand::thread_rng().gen_range(1..=6);
                    let class_bonus = character.get_class_attack_bonus(enemies_outnumbered, &npc.tags);                    
                    let attack_total = attack_roll + class_bonus + equipment_bonus as i32;

                    println!("{} (nivel {}) tira {} + {} + {} = {}\n",
                             character.name, character.level, attack_roll, class_bonus, equipment_bonus, attack_total);

                    if attack_total >= npc.level as i32 {
                        enemies_defeated += 1;
                        println!("¡{} derrota a un {}!\n",
                                 character.name, npc.base.tag);
                    } else {
                        println!(
                            "{} falla el ataque contra el {}.\n",
                            character.name, npc.base.tag
                        );
                    }
                }
            }
        }

        let remaining_after_attack = enemies_remaining - enemies_defeated;

        // Solo los enemigos que quedan pueden contraatacar
        if remaining_after_attack > 0 {
            println!("Contraataque de los enemigos:");
            let enemies_that_can_attack = remaining_after_attack as usize;
            let num_characters = self.characters.len();
            let extra_enemies = if enemies_that_can_attack > num_characters {
                enemies_that_can_attack - num_characters
            } else {
                0
            };

            for (i, character) in self.characters.iter_mut().enumerate() {
                let enemies_for_this_char = if i < extra_enemies {
                    2
                } else if i < enemies_that_can_attack {
                    1
                } else {
                    0
                };

                for _ in 0..enemies_for_this_char {
                    let defense_roll = rand::thread_rng().gen_range(1..=6);
                    let equipment_defense_bonus = character.get_equipment_defense_bonus();
                    let class_defense_bonus = character.get_class_defense_bonus(&npc.tags);
                    let defense_total = defense_roll + equipment_defense_bonus + class_defense_bonus;

                    println!(
                        "{} se defiende con {} + {} + {} = {} contra el {}.\n",
                        character.name, defense_roll, equipment_defense_bonus, class_defense_bonus, defense_total, npc.base.tag
                    );

                    if defense_roll == 1 {
                        character.hit_points -= 1;
                        println!(
                            "¡Fallo crítico! {} recibe 1 punto de daño.\n",
                            character.name
                        );
                    } else if defense_total > npc.level as i32 || defense_roll == 6 {
                        println!(
                            "{} esquiva el ataque del {}.\n",
                            character.name, npc.base.tag
                        );
                    } else {
                        character.hit_points -= 1;
                        println!(
                            "{} recibe 1 punto de daño del {}.\n",
                            character.name, npc.base.tag
                        );
                    }
                }
            }

            // Mostrar el estado actual del grupo
            println!("Estado del grupo después del contraataque:");
            for character in &self.characters {
                println!(
                    "{} ({}, nivel {}): {} PV/{} PV\n",
                    character.name,
                    character.class,
                    character.level,
                    character.hit_points,
                    character.max_hit_points
                );
            }
        }

        remaining_after_attack
    }

    fn handle_level_up(&mut self, input_reader: &mut dyn InputReader, dice: &mut dyn DiceRoller) {
        println!("¡Has alcanzado 10 encuentros superados! Debes subir de nivel a un personaje para continuar.");
        loop {
            println!("Personajes disponibles para subir de nivel:");
            let available_characters: Vec<_> = self.characters.iter()
                .filter(|c| {
                    if let Some(last_leveled) = self.leveled_up_last_time.as_ref() {
                        c.name != *last_leveled
                    } else {
                        true
                    }
                })
                .collect();

            if available_characters.is_empty() {
                println!("No hay personajes disponibles para subir de nivel.");
                self.encounters_won = 0;
                break;
            }

            for character in &available_characters {
                println!("- {} (nivel {})", character.name, character.level);
            }

            println!("Escribe el nombre del personaje que quieres subir de nivel:");
            let input = input_reader.read_line().trim().to_string();

            // Verificar si el personaje ya subió de nivel y si los demás están en nivel 5
            let can_level_up = if let Some(last_leveled) = self.leveled_up_last_time.as_ref() {
                if input.to_lowercase() == last_leveled.to_lowercase() {
                    // Verificar si todos los demás personajes están en nivel 5
                    self.characters.iter()
                        .filter(|other| other.name.to_lowercase() != input.to_lowercase())
                        .all(|other| other.level >= 5)
                } else {
                    true
                }
            } else {
                true
            };

            if !can_level_up {
                println!("Este personaje ya ha subido de nivel anteriormente. Por favor, elige otro personaje.");
                continue;
            }

            if let Some(character) = self.characters.iter_mut()
                .find(|c| c.name.to_lowercase() == input.to_lowercase()) {
                // evaluate if the character levels up
                let level_roll = dice.roll_1d6();
                if level_roll > character.level as u8 {
                    character.level += 1;
                    character.max_hit_points += 1;
                    character.hit_points = character.max_hit_points;
                    self.leveled_up_last_time = Some(character.name.clone());
                    println!("¡{} ha subido al nivel {}!", character.name, character.level);
                } else {
                    println!("¡{} no ha subido de nivel! ¡Más suerte la próxima vez!", character.name);
                }
                self.encounters_won = 0;
                break;
            } else {
                println!("No se encontró ningún personaje con ese nombre. Inténtalo de nuevo.");
            }
        }
    }

    pub fn has_hostile_npcs(&self) -> bool {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                return !location.content.npcs.is_empty() && location.content.npcs.iter()
                    .filter_map(|npc_tag| find_npc(npc_tag))
                    .any(|npc| npc.attitude == Attitude::Hostile && !self.defeated_npcs.contains(&npc.base.tag));
            }
        }
        false
    }

    pub fn execute_equip(&mut self, args: &[&str]) -> bool {
        if args.is_empty() {
            println!("¿Qué quieres equipar?");
            println!("Comandos válidos:");
            println!("- equipar [nombre_personaje] [tipo_equipo]");
            println!("- equipar [tipo_equipo]");
            println!("\nTipos de equipo disponibles:");
            println!("- arma (espada_inicial, hacha_inicial, daga_inicial)");
            println!("- escudo (escudo_inicial)");
            println!("- armadura (armadura_inicial)");
            return false;
        }

        let (character_index, equipment_type) = if args.len() == 1 {
            (0, args[0])
        } else {
            // Buscar el personaje por nombre
            if let Some(index) = self.characters.iter().position(|c| c.name.to_lowercase() == args[0].to_lowercase()) {
                (index, args[1])
            } else {
                println!("No se encontró ningún personaje con ese nombre.");
                println!("\nPersonajes disponibles:");
                for character in &self.characters {
                    println!("- {} ({})", character.name, character.class);
                }
                return false;
            }
        };

        let equipment_type = match equipment_type.to_lowercase().as_str() {
            "arma" | "weapon" => Some(EquipmentType::Weapon(WeaponType::Medium)),
            "escudo" | "shield" => Some(EquipmentType::Shield),
            "armadura" | "armor" => Some(EquipmentType::Armor(ArmorType::Light)),
            _ => {
                println!("Tipo de equipo no válido.");
                println!("Tipos disponibles: arma, escudo, armadura");
                None
            }
        };

        if let Some(equipment_type) = equipment_type {
            let character = &mut self.characters[character_index];
            let inventory = &mut self.inventory;

            if let Some(item_index) = inventory.iter().position(|item| {
                item.is_equipment && item.equipment_type.as_ref().map_or(false, |et| et == &equipment_type)
            }) {
                let item = inventory.remove(item_index);
                if let Some(equipment) = item.to_equipment() {
                    if let Some(unequipped) = character.equip(equipment) {
                        inventory.push(Item::from_equipment(unequipped));
                    }
                }
                true
            } else {
                println!("No tienes ese objeto en tu inventario.");
                println!("\nObjetos equipables disponibles:");
                for item in inventory.iter().filter(|i| i.is_equipment) {
                    if let Some(et) = &item.equipment_type {
                        match et {
                            EquipmentType::Basic => println!("- {} (objeto básico)", item.base.description),
                            EquipmentType::Bow => println!("- {} (arco)", item.base.description),
                            EquipmentType::Weapon(_) => println!("- {} (arma)", item.base.description),
                            EquipmentType::Shield => println!("- {} (escudo)", item.base.description),
                            EquipmentType::Armor(_) => println!("- {} (armadura)", item.base.description),
                        }
                    }
                }
                false
            }
        } else {
            false
        }
    }

    pub fn execute_unequip(&mut self, args: &[&str]) -> bool {
        if args.is_empty() {
            println!("¿Qué quieres desequipar?");
            println!("Comandos válidos:");
            println!("- desequipar [nombre_personaje] [tipo_equipo]");
            println!("- desequipar [tipo_equipo]");
            println!("\nTipos de equipo disponibles:");
            println!("- arma");
            println!("- escudo");
            println!("- armadura");
            return false;
        }

        let (character_index, equipment_type) = if args.len() == 1 {
            (0, args[0])
        } else {
            // Buscar el personaje por nombre
            if let Some(index) = self.characters.iter().position(|c| c.name.to_lowercase() == args[0].to_lowercase()) {
                (index, args[1])
            } else {
                println!("No se encontró ningún personaje con ese nombre.");
                println!("\nPersonajes disponibles:");
                for character in &self.characters {
                    println!("- {} ({})", character.name, character.class);
                }
                return false;
            }
        };

        let equipment_type = match equipment_type.to_lowercase().as_str() {
            "arma" | "weapon" => Some(EquipmentType::Weapon(WeaponType::Medium)),
            "escudo" | "shield" => Some(EquipmentType::Shield),
            "armadura" | "armor" => Some(EquipmentType::Armor(ArmorType::Light)),
            _ => {
                println!("Tipo de equipo no válido.");
                println!("Tipos disponibles: arma, escudo, armadura");
                None
            }
        };

        if let Some(equipment_type) = equipment_type {
            let character = &mut self.characters[character_index];
            if let Some(equipment) = character.unequip(equipment_type) {
                self.inventory.push(Item::from_equipment(equipment));
                true
            } else {
                println!("No tienes ese tipo de equipo equipado.");
                false
            }
        } else {
            false
        }
    }

    pub fn execute_new(&mut self, input: &str) -> String {
        if self.characters.len() >= 3 {
            return "Ya tienes el máximo de personajes permitidos (3).".to_string();
        }

        let existing_names: HashSet<String> = self.characters.iter()
            .map(|c| c.name.clone())
            .collect();

        match parse_new_character(input.to_string(), &existing_names) {
            Ok(character) => {
                self.characters.push(character);
                format!("¡Has creado un nuevo personaje! Ahora tienes {} personajes.", self.characters.len())
            },
            Err(e) => e
        }
    }
}

#[cfg(test)]
mod tests;
