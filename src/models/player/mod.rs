use crate::Character;
use crate::models::object::{Object, ObjectType, LOCATIONS, OBJECTS};
use std::collections::{HashMap, HashSet};
use rand::Rng;

#[derive(Debug)]
pub struct Player<'a> {
    characters: Vec<Character>,
    pub current_location: Option<&'a Object<'a>>,
    pub inventory: Vec<&'a Object<'a>>,
    pub search_attempts: HashMap<String, u32>,  // Sala -> número de intentos
    pub discovered_objects: HashSet<String>,    // Tags de objetos descubiertos
    pub dropped_objects: HashMap<String, &'a Object<'a>>, // tag -> ubicación donde se soltó
}

impl<'a> Player<'a> {
    pub fn new(characters: Vec<Character>) -> Self {
        Self { 
            characters, 
            current_location: None,
            inventory: Vec::new(),
            search_attempts: HashMap::new(),
            discovered_objects: HashSet::new(),
            dropped_objects: HashMap::new(),
        }
    }

    fn set_current_location(&mut self, location: Option<&'a Object<'a>>) {
        self.current_location = location;
    }

    pub fn execute_look(&self) {
        println!("Estas en {}", self.current_location.unwrap().description);
        let objects_in_location = {
            let tag: &str = &self.current_location.unwrap().tag;
            if let Some(location) = LOCATIONS.iter().find(|&loc| loc.tag == tag) {
                OBJECTS.iter()
                    .filter(|&obj| {
                        ((obj.get_location().map_or(false, |loc| loc.tag == location.tag) && 
                            !self.dropped_objects.contains_key(&obj.tag)) ||
                        self.dropped_objects.get(&obj.tag).map_or(false, |&loc| loc.tag == location.tag)) &&
                        !self.inventory.iter().any(|inv_obj| inv_obj.tag == obj.tag)
                    })
                    .filter(|&obj| obj.visible || self.discovered_objects.contains(&obj.tag))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };
        if objects_in_location.len() > 0 {
            println!("Puedes ver:");
            for (i, object) in objects_in_location.iter().enumerate() {
                if i == objects_in_location.len() - 1 {
                    println!("- {}.", object.description);
                } else {
                    println!("- {},", object.description);
                }
            }
        }
    }

    pub fn execute_go(&mut self, location: Option<&'a Object<'a>>) {
        if location.is_some() {
            self.set_current_location(Option::from(location));
            self.execute_look();
        }
    }

    pub fn execute_take(&mut self, object: &'a Object<'a>) -> bool {
        // Check if the object is in the current location
        if let Some(location) = self.current_location {
            let objects_in_location = {
                let tag: &str = &location.tag;
                if let Some(location) = LOCATIONS.iter().find(|&loc| loc.tag == tag) {
                    OBJECTS.iter()
                        .filter(|&obj| {
                            ((obj.get_location().map_or(false, |loc| loc.tag == location.tag) && 
                                !self.dropped_objects.contains_key(&obj.tag)) ||
                            self.dropped_objects.get(&obj.tag).map_or(false, |&loc| loc.tag == location.tag)) &&
                            !self.inventory.iter().any(|inv_obj| inv_obj.tag == obj.tag)
                        })
                        .filter(|&obj| 
                            (obj.visible || self.discovered_objects.contains(&obj.tag)) &&
                            obj.object_type == ObjectType::Item)
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            };

            if objects_in_location.iter().any(|obj| obj.tag == object.tag) {
                // Check if the object is an item
                if object.object_type == ObjectType::Item {
                    self.dropped_objects.remove(&object.tag);
                    self.inventory.push(object);
                    println!("Has cogido {} y lo has añadido a tu inventario.", object.description);
                    return true;
                } else {
                    println!("No puedes coger eso.");
                }
            } else {
                println!("No hay ningún objeto con ese nombre aquí.");
                if !objects_in_location.is_empty() {
                    println!("\nPuedes coger:");
                    for (i, obj) in objects_in_location.iter().enumerate() {
                        if i == objects_in_location.len() - 1 {
                            println!("- {} [{}].", obj.description, obj.tag);
                        } else {
                            println!("- {} [{}],", obj.description, obj.tag);
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
                    println!("- {}.", item.description);
                } else {
                    println!("- {},", item.description);
                }
            }
        }
    }

    pub fn has_item(&self, tag: &str) -> bool {
        self.inventory.iter().any(|item| item.tag == tag)
    }

    pub fn execute_search(&mut self) -> bool {
        if let Some(location) = self.current_location {
            // Obtener el número de intentos en esta sala
            let attempts = self.search_attempts.entry(location.tag.clone()).or_insert(0);
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

            // Reducir por dificultad base (podríamos hacer esto más sofisticado)
            success_chance -= 0;

            // Asegurar que la probabilidad esté entre 5% y 95%
            success_chance = success_chance.max(5).min(95);

            // Generar número aleatorio
            let mut rng = rand::thread_rng();
            let roll = rng.gen_range(1..=100);

            if roll <= success_chance {
                // Éxito en la búsqueda
                println!("¡Has encontrado algo!");
                
                // Buscar objetos ocultos en la sala
                let objects_in_location = {
                    let tag: &str = &location.tag;
                    if let Some(location) = LOCATIONS.iter().find(|&loc| loc.tag == tag) {
                        OBJECTS.iter()
                            .filter(|&obj| obj.get_location() == Some(location) && !obj.visible && !self.discovered_objects.contains(&obj.tag))
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    }
                };
                let mut found_something = false;
                for object in objects_in_location {
                    println!("Has descubierto {}", object.description);
                    self.discovered_objects.insert(object.tag.clone());
                    found_something = true;
                }

                if !found_something {
                    println!("No hay nada más que encontrar aquí.");
                }

                // Reiniciar contador de intentos para esta sala
                self.search_attempts.remove(&location.tag);
                return true;
            } else {
                println!("No encuentras nada especial...");
            }
        }
        false
    }

    pub fn execute_drop(&mut self, object: &'a Object<'a>) -> bool {
        if let Some(location) = self.current_location {
            if let Some(index) = self.inventory.iter().position(|&obj| obj.tag == object.tag) {
                self.inventory.remove(index);
                self.dropped_objects.insert(object.tag.clone(), location);
                println!("Has soltado {}.", object.description);
                return true;
            } else {
                println!("No tienes ese objeto en tu inventario.");
            }
        }
        false
    }
}