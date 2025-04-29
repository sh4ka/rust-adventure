use std::fmt::{Display, Formatter};
use crate::location::Location;
use crate::parsexec::Command;

#[derive(Debug)]
pub enum Class {
    Fighter
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { Class::Fighter => {std::write!(f,"Guerrero")} }
    }
}

#[derive(Debug)]
pub struct Character {
    pub(crate) class: Class,
    hit_points: u32,
    level: u32,
}

fn get_hit_points(class: &Class, level: u32) -> u32 {
    match class { Class::Fighter => {level + 6} }
}

impl Character {
    pub fn new(class: Class) -> Character {
        let hit_points: u32 = get_hit_points(&class, 1);
        Character {
            class,
            hit_points,
            level: 1
        }
    }
}

pub fn parse_new_character(input: &str) -> Result<Character, String> {
    let input = input.trim().to_lowercase();
    let mut words = input.split_whitespace();

    match words.next() {
        Some("guerrero") => Ok(Character::new(Class::Fighter)),
        _ => Result::Err("Clase incorrecta.".to_string()),
    }
}