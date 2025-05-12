use crate::Character;
use crate::models::object::{Location, Item, NPC, Passage, find_location, find_npc, find_item_in_location, find_passage, find_item, PASSAGES, Attitude, NPCTag};
use crate::models::character::{Equipment, EquipmentType, WeaponType, ArmorType, Class};
use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::io::{self, Write};

use super::enemy;

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
}

impl Player {
    pub fn new(characters: Vec<Character>) -> Self {
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

    pub fn execute_look(&self) -> String {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                let mut response = format!("Estás en {}\n", location.base.description);
                response.push_str(&format!("\n{}\n", location.base.long_description));

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
                    response.push_str("\nHay:\n");
                    for item in visible_items {
                        response.push_str(&format!("- {}\n", item.base.description));
                    }
                    for item in dropped_items {
                        response.push_str(&format!("- {}\n", item.base.description));
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
                        response.push_str(&format!("- {}{}\n", npc.base.description, attitude));
                    }
                }
                
                return response;
            }
        }
        "No estás en ninguna ubicación.".to_string()
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
                                // Verificar si hay un pasaje que conecte las ubicaciones
                                if let Some(passage) = PASSAGES.values().find(|p| 
                                    (p.from == *current_location && p.to == tag) ||
                                    (p.from == tag && p.to == *current_location)
                                ) {
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
                                            if let Ok(_) = std::io::stdin().read_line(&mut input) {
                                                let input = input.trim().to_lowercase();
                                                if input == "cancelar" {
                                                    return "Has decidido no intentar resolver el acertijo.".to_string();
                                                }
                                                if input != answer.to_lowercase() {
                                                    return "Respuesta incorrecta. La puerta permanece cerrada.".to_string();
                                                }
                                            }
                                        }
                                    }

                                    // Si llegamos aquí, el jugador puede pasar
                                    self.set_current_location(Some(tag.to_string()));
                                    return self.execute_look();
                                } else {
                                    // Si no hay pasaje, permitir el movimiento directo
                                    self.set_current_location(Some(tag.to_string()));
                                    return self.execute_look();
                                }
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
                        return self.execute_look();
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
            let mut seen_items = HashSet::new();

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

                if !is_equipped && seen_items.insert(item.base.tag.clone()) {
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
            for (index, character) in self.characters.iter().enumerate() {
                if character.weapon.is_some() || character.shield.is_some() || 
                   character.armor.is_some() || character.bow.is_some() {
                    if !has_equipped {
                        println!("\nObjetos equipados:");
                        has_equipped = true;
                    }
                    if let Some(weapon) = &character.weapon {
                        println!("- {} (equipado por Aventurero {} - {})", weapon.name, index + 1, character.class);
                    }
                    if let Some(shield) = &character.shield {
                        println!("- {} (equipado por Aventurero {} - {})", shield.name, index + 1, character.class);
                    }
                    if let Some(armor) = &character.armor {
                        println!("- {} (equipado por Aventurero {} - {})", armor.name, index + 1, character.class);
                    }
                    if let Some(bow) = &character.bow {
                        println!("- {} (equipado por Aventurero {} - {})", bow.name, index + 1, character.class);
                    }
                }
            }

            if non_equipped_items.is_empty() && !has_equipped {
                println!("Tu inventario está vacío.");
            }
        }
    }

    pub fn execute_status(&self) -> String {
        let mut response = String::new();
        response.push_str("Estado del grupo:\n");
        
        for (i, character) in self.characters.iter().enumerate() {
            response.push_str(&format!(
                "Aventurero {} ({}, nivel {}): {} PV/{} PV\n",
                i + 1,
                character.class,
                character.level,
                character.hit_points,
                character.max_hit_points
            ));
        }
        
        response
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

    pub fn execute_attack(&mut self, target_tag: &str) -> String {
        let mut response = String::new();
        
        // Si el comando es "continuar", no mostrar el mensaje de inicio
        if target_tag != "continuar" {
            response.push_str("¡Comienza el combate!\n");
        }

        // Obtener NPCs hostiles en la ubicación actual
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                let hostile_npcs: Vec<&NPC> = location.content.npcs.iter()
                    .filter_map(|npc_tag| find_npc(npc_tag))
                    .filter(|npc| npc.attitude == Attitude::Hostile && !self.defeated_npcs.contains(&npc.base.tag))
                    .collect();

                if hostile_npcs.is_empty() {
                    self.current_combat_enemies = None;
                    return "No hay enemigos para atacar aquí.".to_string();
                }

                // Usar el número de enemigos guardado o calcular el total inicial
                let mut enemies_remaining = if let Some(remaining) = self.current_combat_enemies {
                    remaining
                } else {
                    let total_enemies: u8 = hostile_npcs.iter().map(|npc| npc.count).sum();
                    total_enemies
                };

                // Si es una acción de combate (1-4), procesar la acción
                if target_tag == "1" {
                    // Continuar el combate
                    self.current_combat_enemies = Some(enemies_remaining);
                    return self.execute_attack("continuar");
                } else if target_tag == "2" {
                    // Huir
                    self.current_combat_enemies = None;
                    return "Has huido del combate.".to_string();
                } else if target_tag == "3" {
                    return "Función de usar objetos aún no implementada.".to_string();
                } else if target_tag == "4" {
                    return self.execute_status();
                }

                // Rondas de combate
                while enemies_remaining > 0 {
                    // Fase de ataque de los aventureros
                    response.push_str("\nAtaque de los aventureros:\n");
                    let mut enemies_defeated = 0;
                    let enemies_outnumbered = self.characters.len() > enemies_remaining as usize;
                    
                    for (i, character) in self.characters.iter_mut().enumerate() {
                        if enemies_remaining == 0 {
                            break;
                        }

                        // Verificar si el personaje tiene un arma equipada
                        match character.get_attack_bonus() {
                            None => {
                                response.push_str(&format!(
                                    "Aventurero {} ({}) no puede atacar porque no tiene un arma equipada.\n",
                                    i + 1, character.class
                                ));
                                continue;
                            },
                            Some(equipment_bonus) => {
                                let attack_roll = rand::thread_rng().gen_range(1..=6);
                                let npc = &hostile_npcs[0]; // Todos los enemigos son del mismo tipo
                                let class_bonus = character.get_class_bonus(enemies_outnumbered, npc.tags.first());
                                let attack_total = attack_roll + class_bonus + equipment_bonus;
                                
                                let bonus_message = if npc.has_tag(&NPCTag::Orc) && matches!(character.class, Class::Elf) {
                                    format!("{} + 1 (vs orcos)", character.level)
                                } else {
                                    character.level.to_string()
                                };

                                response.push_str(&format!(
                                    "Aventurero {} (nivel {}) tira {} + {} + {} = {}\n",
                                    i + 1, character.level, attack_roll, bonus_message, equipment_bonus, attack_total
                                ));

                                if attack_total >= npc.level as i32 {
                                    enemies_defeated += 1;
                                    enemies_remaining -= 1;
                                    response.push_str(&format!(
                                        "¡Aventurero {} derrota a un {}!\n",
                                        i + 1, npc.base.tag
                                    ));
                                } else {
                                    response.push_str(&format!(
                                        "Aventurero {} falla el ataque contra el {}.\n",
                                        i + 1, npc.base.tag
                                    ));
                                }
                            }
                        }
                    }

                    // Solo los enemigos que quedan pueden contraatacar
                    if enemies_remaining > 0 {
                        response.push_str("\nContraataque de los enemigos:\n");
                        let enemies_that_can_attack = enemies_remaining as usize;
                        let num_characters = self.characters.len();
                        
                        // Calcular cuántos enemigos adicionales hay después de asignar uno a cada personaje
                        let extra_enemies = if enemies_that_can_attack > num_characters {
                            enemies_that_can_attack - num_characters
                        } else {
                            0
                        };

                        // Para cada personaje
                        for (i, character) in self.characters.iter_mut().enumerate() {
                            // Calcular cuántos enemigos atacan a este personaje
                            let enemies_for_this_char = if i < extra_enemies {
                                2 // Este personaje enfrenta a 2 enemigos
                            } else if i < enemies_that_can_attack {
                                1 // Este personaje enfrenta a 1 enemigo
                            } else {
                                0 // Este personaje no enfrenta enemigos
                            };

                            // Procesar los ataques de los enemigos asignados a este personaje
                            for _ in 0..enemies_for_this_char {
                                let npc = &hostile_npcs[0]; // Todos los enemigos son del mismo tipo
                                let defense_roll = rand::thread_rng().gen_range(1..=6);
                                let defense_bonus = character.get_defense_bonus(&npc.tags);
                                let defense_total = defense_roll + defense_bonus;
                                
                                response.push_str(&format!(
                                    "Aventurero {} se defiende con {} + {} = {} contra el {}.\n",
                                    i + 1, defense_roll, defense_bonus, defense_total, npc.base.tag
                                ));

                                if defense_roll == 1 {
                                    // Fallo crítico
                                    character.hit_points -= 1;
                                    response.push_str(&format!(
                                        "¡Fallo crítico! Aventurero {} recibe 1 punto de daño.\n",
                                        i + 1
                                    ));
                                } else if defense_total > npc.level as i32 || defense_roll == 6 {
                                    // Defensa exitosa
                                    response.push_str(&format!(
                                        "Aventurero {} esquiva el ataque del {}.\n",
                                        i + 1, npc.base.tag
                                    ));
                                } else {
                                    // Ataque exitoso
                                    character.hit_points -= 1;
                                    response.push_str(&format!(
                                        "Aventurero {} recibe 1 punto de daño del {}.\n",
                                        i + 1, npc.base.tag
                                    ));
                                }
                            }
                        }

                        // Guardar el número de enemigos restantes
                        self.current_combat_enemies = Some(enemies_remaining);

                        // Mostrar el estado actual del grupo
                        response.push_str("\nEstado del grupo después del contraataque:\n");
                        for (i, character) in self.characters.iter().enumerate() {
                            response.push_str(&format!(
                                "Aventurero {} ({}, nivel {}): {} PV/{} PV\n",
                                i + 1,
                                character.class,
                                character.level,
                                character.hit_points,
                                character.max_hit_points
                            ));
                        }

                        // Interrumpir el combate para que el jugador decida qué hacer
                        response.push_str("\n¿Qué quieres hacer?\n");
                        response.push_str("1. Continuar el combate\n");
                        response.push_str("2. Huir\n");
                        response.push_str("3. Usar un objeto\n");
                        response.push_str("4. Ver estado detallado\n");
                        return response;
                    }
                }

                // Combate terminado, limpiar el estado y marcar enemigos como derrotados
                self.current_combat_enemies = None;
                for npc in hostile_npcs {
                    self.defeated_npcs.insert(npc.base.tag.clone());
                }

                response.push_str("\n¡Combate terminado!\n");
                return response;
            }
        }
        "No estás en ninguna ubicación.".to_string()
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
            println!("- equipar [número_personaje] [tipo_equipo]");
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
            match args[0].parse::<usize>() {
                Ok(index) if index > 0 && index <= self.characters.len() => (index - 1, args[1]),
                _ => {
                    println!("Número de personaje inválido.");
                    return false;
                }
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
            println!("- desequipar [número_personaje] [tipo_equipo]");
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
            match args[0].parse::<usize>() {
                Ok(index) if index > 0 && index <= self.characters.len() => (index - 1, args[1]),
                _ => {
                    println!("Número de personaje inválido.");
                    return false;
                }
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

    pub fn execute_talk_to_npc(&mut self, npc_tag: &str) {
        if let Some(npc) = find_npc(npc_tag) {
            let is_in_location = self.current_location.as_ref().map_or(false, |current| current == &npc.location);
            if is_in_location {
                println!("Hablas con {}.", npc.base.description);
                if npc.attitude == Attitude::Hostile {
                    println!("{} te ataca!", npc.base.description);
                    self.execute_attack(npc_tag);
                } else {
                    println!("{} te saluda amistosamente.", npc.base.description);
                }
            } else {
                println!("No veo a {} por aquí.", npc.base.description);
            }
        } else {
            println!("No veo a nadie con ese nombre.");
        }
    }

    pub fn get_defense_bonus(&self, character_index: usize) -> i32 {
        if let Some(character) = self.characters.get(character_index) {
            character.get_defense_bonus(&[])
        } else {
            0
        }
    }
}