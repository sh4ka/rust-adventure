use std::fmt::{Display, Formatter};
use crate::models::object::Item;

#[derive(Debug, Clone, PartialEq)]
pub enum Class {
    Fighter,
    Cleric,
    Rogue,
    Wizard,
    Barbarian,
    Elf,
    Dwarf,
    Halfling,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeaponType {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArmorType {
    Light,
    Heavy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EquipmentType {
    Weapon(WeaponType),
    Shield,
    Armor(ArmorType),
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub name: String,
    pub equipment_type: EquipmentType,
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
            Class::Halfling => write!(f, "Mediano"),
        }
    }
}

#[derive(Debug)]
pub struct Character {
    pub class: Class,
    pub(crate) hit_points: u32,
    pub(crate) max_hit_points: u32,
    pub level: u32,
    pub weapon: Option<Equipment>,
    pub shield: Option<Equipment>,
    pub armor: Option<Equipment>,
}

impl Character {
    pub fn new(class: Class) -> Character {
        let max_hit_points = Self::calculate_hit_points(&class, 1);
        Character {
            class: class.clone(),
            hit_points: max_hit_points,
            max_hit_points,
            level: 1,
            weapon: None,
            shield: None,
            armor: None,
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

    pub fn equip(&mut self, equipment: Equipment) -> Option<Equipment> {
        match &equipment.equipment_type {
            EquipmentType::Weapon(_) => self.weapon.replace(equipment),
            EquipmentType::Shield => self.shield.replace(equipment),
            EquipmentType::Armor(_) => self.armor.replace(equipment),
        }
    }

    pub fn unequip(&mut self, equipment_type: EquipmentType) -> Option<Equipment> {
        match equipment_type {
            EquipmentType::Weapon(_) => self.weapon.take(),
            EquipmentType::Shield => self.shield.take(),
            EquipmentType::Armor(_) => self.armor.take(),
        }
    }

    pub fn get_attack_bonus(&self) -> i32 {
        let weapon_bonus = self.weapon.as_ref().map_or(0, |w| w.get_bonus());
        let shield_bonus = self.shield.as_ref().map_or(0, |s| s.get_bonus());
        weapon_bonus + shield_bonus
    }

    pub fn get_defense_bonus(&self) -> i32 {
        let armor_bonus = self.armor.as_ref().map_or(0, |a| a.get_bonus());
        let shield_bonus = self.shield.as_ref().map_or(0, |s| s.get_bonus());
        armor_bonus + shield_bonus
    }

    pub fn get_equipment_bonus(&self) -> Option<i32> {
        if self.weapon.is_none() {
            return None;
        }
        let weapon_bonus = self.weapon.as_ref().map_or(0, |w| w.get_bonus());
        let shield_bonus = self.shield.as_ref().map_or(0, |s| s.get_bonus());
        Some(weapon_bonus + shield_bonus)
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

impl Equipment {
    pub fn new(name: String, equipment_type: EquipmentType) -> Self {
        Self {
            name,
            equipment_type,
        }
    }

    pub fn get_bonus(&self) -> i32 {
        match &self.equipment_type {
            EquipmentType::Weapon(weapon_type) => match weapon_type {
                WeaponType::Light => -1,
                WeaponType::Medium => 0,
                WeaponType::Heavy => 1,
            },
            EquipmentType::Shield => 1,
            EquipmentType::Armor(armor_type) => match armor_type {
                ArmorType::Light => 1,
                ArmorType::Heavy => 2,
            },
        }
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