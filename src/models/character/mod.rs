use std::fmt::{Display, Formatter};

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
    pub(crate) hit_points: u32,
    pub(crate) max_hit_points: u32,
    pub level: u32,
}

fn get_hit_points(class: &Class, level: u32) -> u32 {
    match class { Class::Fighter => {level * 2 + 6} }
}

impl Character {
    pub fn new(class: Class) -> Character {
        let max_hit_points = get_hit_points(&class, 1);
        Character {
            class,
            hit_points: max_hit_points,
            max_hit_points,
            level: 1
        }
    }

    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let actual_damage = damage.min(self.hit_points);
        self.hit_points = self.hit_points.saturating_sub(damage);
        actual_damage
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
}

pub fn parse_new_character(input: String) -> Result<Character, String> {
    let input = input.trim().to_lowercase();
    let mut words = input.split_whitespace();

    match words.next() {
        Some("guerrero") => Ok(Character::new(Class::Fighter)),
        _ => Err("Clase incorrecta.".to_string()),
    }
}