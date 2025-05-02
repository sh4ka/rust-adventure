use crate::Character;
use crate::models::object::{Location, Item, NPC, Passage, find_location, find_item, get_items_in_location, get_npcs_in_location, get_passages_from_location};
use std::collections::{HashMap, HashSet};
use rand::Rng;

#[derive(Debug)]
pub struct Player {
    characters: Vec<Character>,
    pub current_location: Option<String>,  // Tag de la ubicación actual
    pub inventory: Vec<String>,           // Tags de los items en el inventario
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
        self.current_location = location_tag;
    }

    pub fn execute_look(&self) {
        if let Some(location_tag) = &self.current_location {
            if let Some(location) = find_location(location_tag) {
                println!("Estas en {}", location.base.description);
                
                // Obtener items en la ubicación
                let items = get_items_in_location(location_tag);
                let visible_items: Vec<_> = items.iter()
                    .filter(|item| {
                        // El item está en la ubicación actual si:
                        // 1. Está en su ubicación original y no ha sido soltado en otro lugar
                        // 2. Ha sido soltado en la ubicación actual
                        ((item.location.as_ref().map_or(false, |loc| loc == location_tag) && 
                            !self.dropped_items.contains_key(&item.base.tag)) ||
                        self.dropped_items.get(&item.base.tag).map_or(false, |loc| loc == location_tag)) &&
                        !self.inventory.contains(&item.base.tag)
                    })
                    .filter(|item| item.base.visible || self.discovered_items.contains(&item.base.tag))
                    .collect();

                // Obtener NPCs en la ubicación
                let npcs = get_npcs_in_location(location_tag);
                let visible_npcs: Vec<_> = npcs.iter()
                    .filter(|npc| npc.base.visible)
                    .collect();

                // Mostrar items y NPCs
                if !visible_items.is_empty() || !visible_npcs.is_empty() {
                    println!("Puedes ver:");
                    
                    // Mostrar items
                    for (i, item) in visible_items.iter().enumerate() {
                        if i == visible_items.len() - 1 && visible_npcs.is_empty() {
                            println!("- {}.", item.base.description);
                        } else {
                            println!("- {},", item.base.description);
                        }
                    }

                    // Mostrar NPCs
                    for (i, npc) in visible_npcs.iter().enumerate() {
                        if i == visible_npcs.len() - 1 {
                            println!("- {}.", npc.base.description);
                        } else {
                            println!("- {},", npc.base.description);
                        }
                    }
                }
            }
        }
    }

    pub fn execute_go(&mut self, location_tag: Option<String>) {
        match location_tag {
            Some(tag) => {
                if let Some(current_location) = &self.current_location {
                    if let Some(location) = find_location(current_location) {
                        // Verificar si la ubicación destino está conectada
                        if location.connections.contains(&tag) {
                            if find_location(&tag).is_some() {
                                self.set_current_location(Some(tag));
                                self.execute_look();
                            } else {
                                println!("No puedes ir allí.");
                                self.show_available_locations(location);
                            }
                        } else {
                            println!("No hay un camino que lleve a ese lugar desde aquí.");
                            self.show_available_locations(location);
                        }
                    }
                } else {
                    // Si no hay ubicación actual, permitir el movimiento inicial
                    if find_location(&tag).is_some() {
                        self.set_current_location(Some(tag));
                        self.execute_look();
                    } else {
                        println!("No puedes ir allí.");
                        println!("\nPuedes ir a cualquiera de estas ubicaciones:");
                        for (tag, location) in crate::models::object::LOCATIONS.iter() {
                            if location.base.visible {
                                println!("- {} ({})", location.base.description, tag);
                            }
                        }
                    }
                }
            }
            None => {
                if let Some(current_location) = &self.current_location {
                    if let Some(location) = find_location(current_location) {
                        println!("¿Ir a dónde?");
                        self.show_available_locations(location);
                    }
                } else {
                    println!("¿Ir a dónde?");
                    println!("\nPuedes ir a cualquiera de estas ubicaciones:");
                    for (tag, location) in crate::models::object::LOCATIONS.iter() {
                        if location.base.visible {
                            println!("- {} ({})", location.base.description, tag);
                        }
                    }
                }
            }
        }
    }

    fn show_available_locations(&self, location: &Location) {
        println!("\nPuedes ir a:");
        let mut has_connections = false;
        for connection in &location.connections {
            if let Some(connected_location) = find_location(connection) {
                if connected_location.base.visible {
                    println!("- {} ({})", connected_location.base.description, connection);
                    has_connections = true;
                }
            }
        }
        if !has_connections {
            println!("No hay salidas disponibles desde aquí.");
        }
    }

    pub fn execute_take(&mut self, item_tag: &str) -> bool {
        if let Some(location_tag) = &self.current_location {
            // Obtener items en la ubicación actual
            let items = get_items_in_location(location_tag);
            let available_items: Vec<_> = items.iter()
                .filter(|item| {
                    ((item.location.as_ref().map_or(false, |loc| loc == location_tag) && 
                        !self.dropped_items.contains_key(&item.base.tag)) ||
                    self.dropped_items.get(&item.base.tag).map_or(false, |loc| loc == location_tag)) &&
                    !self.inventory.contains(&item.base.tag)
                })
                .filter(|item| 
                    (item.base.visible || self.discovered_items.contains(&item.base.tag)))
                .collect();

            if let Some(item) = available_items.iter().find(|item| item.base.tag == item_tag) {
                // Si el item estaba en su ubicación original, lo removemos de dropped_items
                self.dropped_items.remove(&item.base.tag);
                self.inventory.push(item.base.tag.clone());
                println!("Has cogido {} y lo has añadido a tu inventario.", item.base.description);
                return true;
            } else {
                println!("No hay ningún objeto con ese nombre aquí.");
                if !available_items.is_empty() {
                    println!("\nPuedes coger:");
                    for (i, item) in available_items.iter().enumerate() {
                        if i == available_items.len() - 1 {
                            println!("- {} [{}].", item.base.description, item.base.tag);
                        } else {
                            println!("- {} [{}],", item.base.description, item.base.tag);
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
            for (i, item_tag) in self.inventory.iter().enumerate() {
                if let Some(item) = find_item(item_tag) {
                    if i == self.inventory.len() - 1 {
                        println!("- {}.", item.base.description);
                    } else {
                        println!("- {},", item.base.description);
                    }
                }
            }
        }
    }

    pub fn has_item(&self, tag: &str) -> bool {
        self.inventory.contains(&tag.to_string())
    }

    pub fn execute_search(&mut self) -> bool {
        if let Some(location_tag) = &self.current_location {
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
                let items = get_items_in_location(location_tag);
                let hidden_items: Vec<_> = items.iter()
                    .filter(|item| !item.base.visible && !self.discovered_items.contains(&item.base.tag))
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
        false
    }

    pub fn execute_drop(&mut self, item_tag: &str) -> bool {
        if let Some(location_tag) = &self.current_location {
            if let Some(index) = self.inventory.iter().position(|tag| tag == item_tag) {
                // Remover el item del inventario
                self.inventory.remove(index);
                // Añadir el item a dropped_items con la ubicación actual
                self.dropped_items.insert(item_tag.to_string(), location_tag.clone());
                if let Some(item) = find_item(item_tag) {
                    println!("Has soltado {}.", item.base.description);
                }
                return true;
            } else {
                println!("No tienes ese objeto en tu inventario.");
            }
        }
        false
    }
}