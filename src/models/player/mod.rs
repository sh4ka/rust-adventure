use crate::Character;
use crate::models::object::{find_objects_by_location, Object, ObjectType};

#[derive(Debug)]
pub struct Player<'a> {
    characters: Vec<Character>,
    pub current_location: Option<&'a Object<'a>>,
    inventory: Vec<&'a Object<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(characters: Vec<Character>) -> Self {
        Self { 
            characters, 
            current_location: None,
            inventory: Vec::new(),
        }
    }

    fn set_current_location(&mut self, location: Option<&'a Object<'a>>) {
        self.current_location = location;
    }

    pub fn execute_look(&self) {
        println!("Estas en {}", self.current_location.unwrap().description);
        let objects_in_location = find_objects_by_location(&self.current_location.unwrap().tag);
        if objects_in_location.len() > 0 {
            println!("Puedes ver:");
            for object in objects_in_location {
                println!("- {}", object.description);
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
            let objects_in_location = find_objects_by_location(&location.tag);
            if objects_in_location.contains(&object) {
                // Check if the object is an item
                if object.object_type == ObjectType::Item {
                    self.inventory.push(object);
                    println!("Has cogido {} y lo has añadido a tu inventario.", object.description);
                    return true;
                } else {
                    println!("No puedes coger eso.");
                }
            } else {
                println!("No hay {} aquí.", object.description);
            }
        }
        false
    }

    pub fn execute_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Tu inventario está vacío.");
        } else {
            println!("Tu inventario contiene:");
            for item in &self.inventory {
                println!("- {}", item.description);
            }
        }
    }

    pub fn has_item(&self, tag: &str) -> bool {
        self.inventory.iter().any(|item| item.tag == tag)
    }
}