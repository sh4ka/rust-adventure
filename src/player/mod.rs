use crate::character::Character;
use crate::location::{Location, LOCATIONS};

#[derive(Debug)]
pub struct Player {
    pub characters: Vec<Character>,
    pub current_location: u32
}

impl Player {
    pub fn new(characters: Vec<Character>) -> Self {
        Self { characters, current_location: 0 }
    }

    pub fn set_current_location(&mut self, location: u32) {
        self.current_location = location;
    }
}