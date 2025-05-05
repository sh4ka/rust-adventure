use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Class {
    Fighter,
    Cleric,
    Rogue,
    Wizard,
    Barbarian,
    Elf,
    Dwarf,
    Halfling
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Class::Fighter => write!(f, "Guerrero"),
            Class::Cleric => write!(f, "Clérigo"),
            Class::Rogue => write!(f, "Pícaro"),
            Class::Wizard => write!(f, "Mago"),
            Class::Barbarian => write!(f, "Bárbaro"),
            Class::Elf => write!(f, "Elfo"),
            Class::Dwarf => write!(f, "Enano"),
            Class::Halfling => write!(f, "Mediano")
        }
    }
}

#[derive(Debug)]
pub struct Character {
    pub(crate) class: Class,
    pub(crate) hit_points: u32,
    pub(crate) max_hit_points: u32,
    pub level: u32,
}

impl Character {
    pub fn new(class: Class) -> Character {
        let max_hit_points = Self::calculate_hit_points(&class, 1);
        Character {
            class,
            hit_points: max_hit_points,
            max_hit_points,
            level: 1
        }
    }

    fn calculate_hit_points(class: &Class, level: u32) -> u32 {
        match class { 
            Class::Fighter => level + 6,    // Guerrero: más puntos de vida
            Class::Cleric => level + 4,     // Clérigo: puntos de vida medios
            Class::Rogue => level + 3,      // Pícaro: menos puntos de vida
            Class::Wizard => level + 2,     // Mago: menos puntos de vida
            Class::Barbarian => level + 7,  // Bárbaro: más puntos de vida
            Class::Elf => level + 4,        // Elfo: puntos de vida medios
            Class::Dwarf => level + 5,      // Enano: puntos de vida altos
            Class::Halfling => level + 3    // Mediano: menos puntos de vida
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
        Some("clerigo") | Some("clérigo") => Ok(Character::new(Class::Cleric)),
        Some("picaro") | Some("pícaro") => Ok(Character::new(Class::Rogue)),
        Some("mago") => Ok(Character::new(Class::Wizard)),
        Some("barbaro") | Some("bárbaro") => Ok(Character::new(Class::Barbarian)),
        Some("elfo") => Ok(Character::new(Class::Elf)),
        Some("enano") => Ok(Character::new(Class::Dwarf)),
        Some("mediano") | Some("halfling") => Ok(Character::new(Class::Halfling)),
        _ => Err("Clase incorrecta.".to_string()),
    }
}