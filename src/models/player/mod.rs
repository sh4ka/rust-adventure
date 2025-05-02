use crate::Character;
use crate::models::object::{Location, Item, NPC, Passage, find_location, find_npc, find_item_in_location, find_passage, PASSAGES};
use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Player {
    characters: Vec<Character>,
    pub current_location: Option<String>,  // Tag de la ubicación actual
    pub inventory: Vec<Item>,           // Items en el inventario
    pub search_attempts: HashMap<String, u32>,  // Sala -> número de intentos
    pub discovered_items: HashSet<String>,    // Tags de items descubiertos
    pub dropped_items: HashMap<String, String>, // tag -> ubicación donde se soltó
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
        if let Some(location) = &self.current_location {
            if let Some(loc) = find_location(location) {
                let mut response = format!("Estás en {}\n", loc.base.description);
                
                // Mostrar descripción larga
                response.push_str(&format!("\n{}\n", loc.base.long_description));
                
                // Contar items y npcs visibles
                let visible_items: Vec<_> = loc.content.items.iter()
                    .filter(|item| item.base.visible)
                    .collect();
                
                let visible_npcs: Vec<_> = loc.content.npcs.iter()
                    .filter_map(|npc_tag| find_npc(npc_tag))
                    .filter(|npc| npc.base.visible)
                    .collect();

                // Mostrar items y npc en la ubicación solo si hay visibles
                if !visible_items.is_empty() || !visible_npcs.is_empty() {
                    response.push_str("\nHay:\n");
                    for item in visible_items {
                        response.push_str(&format!("- {}\n", item.base.description));
                    }
                    for npc in visible_npcs {
                        response.push_str(&format!("- {}\n", npc.base.description));
                    }
                }
                
                return response;
            }
        }
        "No estás en ninguna ubicación.".to_string()
    }

    pub fn execute_go(&mut self, location_tag: Option<&str>) -> String {
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
                    if item.base.visible || self.discovered_items.contains(&item.base.tag) {
                        // No podemos modificar location.content directamente
                        // En su lugar, podríamos mantener un registro de items tomados en el Player
                        self.inventory.push(item.clone());
                        println!("Has cogido {} y lo has añadido a tu inventario.", item.base.description);
                        return true;
                    }
                }
                println!("No hay ningún objeto con ese nombre aquí.");
                if !location.content.items.is_empty() {
                    println!("\nPuedes coger:");
                    for (i, item) in location.content.items.iter().enumerate() {
                        if item.base.visible || self.discovered_items.contains(&item.base.tag) {
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
            println!("Tu inventario contiene:");
            for (i, item) in self.inventory.iter().enumerate() {
                if i == self.inventory.len() - 1 {
                    println!("- {}.", item.base.description);
                } else {
                    println!("- {},", item.base.description);
                }
            }
        }
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
                    // Éxito en la búsqueda
                    println!("¡Has encontrado algo!");
                    
                    // Buscar items ocultos en la sala
                    let hidden_items: Vec<_> = location.content.items.iter()
                        .filter(|item| {
                            !item.base.visible && !self.discovered_items.contains(&item.base.tag)
                        })
                        .collect();

                    let mut found_something = false;
                    for item in hidden_items {
                        println!("Has descubierto {}", item.base.description);
                        self.discovered_items.insert(item.base.tag.clone());
                        found_something = true;
                    }

                    if !found_something {
                        println!("No hay nada más que encontrar aquí.");
                    }

                    // Reiniciar contador de intentos para esta sala
                    self.search_attempts.remove(location_tag);
                    return true;
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
                    // No podemos modificar location.content directamente
                    // En su lugar, podríamos mantener un registro de items soltados en el Player
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
                        if connected_location.base.visible {
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
}