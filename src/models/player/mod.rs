use crate::Character;
use crate::models::object::{find_objects_by_location, Object};

#[derive(Debug)]
pub struct Player<'a> {
    characters: Vec<Character>,
    pub current_location: Option<&'a Object<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(characters: Vec<Character>) -> Self {
        Self { characters, current_location: None }
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
}