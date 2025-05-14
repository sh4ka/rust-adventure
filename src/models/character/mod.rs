use std::fmt::{Display, Formatter};
use crate::models::object::{Item, NPCTag};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterTrait {
    // Traits de combate
    Strong,          // Fuerte: +1 al daño con armas pesadas
    Agile,           // Ágil: +1 a la defensa con armadura ligera
    Tough,           // Resistente: +1 PV por nivel
    Precise,         // Preciso: +1 al ataque con armas a distancia
    ShieldMaster,    // Maestro de escudo: +1 a la defensa con escudo
    
    // Traits de exploración
    Stealthy,        // Sigiloso: +20% probabilidad de encontrar objetos ocultos
    Perceptive,      // Perspicaz: Puede ver objetos ocultos sin buscarlos
    Lucky,           // Afortunado: +10% probabilidad de éxito en todas las acciones
    
    // Traits de clase específicos
    Spellcaster,     // Lanzador de conjuros: Puede usar objetos mágicos
    Healer,          // Sanador: Puede usar pociones de forma más efectiva
    Thief,           // Ladrón: Puede abrir cerraduras sin ganzúas
    Berserker,       // Berserker: +2 al ataque cuando está herido
    NaturalArmor,    // Armadura natural: +1 a la defensa sin armadura
    ForestFriend,    // Amigo del bosque: Bonus en zonas naturales
    MountainBorn,    // Nacido en la montaña: Bonus en zonas montañosas
    Nimble,          // Ágil: Puede esquivar ataques más fácilmente
    // traits de clase y raza
    Warrior,
    Wizard,
    Rogue,
    Cleric,
    Barbarian,
    Dwarf,           // Enano: +1 a la defensa
    Elf,             // Elfo: +1 a la defensa
    Halfling,        // Mediano: +1 a la defensa
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
    Basic,
    Weapon(WeaponType),
    Shield,
    Armor(ArmorType),
    Bow,
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

impl Class {
    pub fn get_traits(&self) -> HashSet<CharacterTrait> {
        let mut traits = HashSet::new();
        match self {
            Class::Fighter => {
                traits.insert(CharacterTrait::Warrior);
            },
            Class::Cleric => {
                traits.insert(CharacterTrait::Cleric);
            },
            Class::Rogue => {
                traits.insert(CharacterTrait::Rogue);
            },
            Class::Wizard => {
                traits.insert(CharacterTrait::Wizard);
            },
            Class::Barbarian => {
                traits.insert(CharacterTrait::Barbarian);
            },
            Class::Elf => {
                traits.insert(CharacterTrait::Elf);
            },
            Class::Dwarf => {
                traits.insert(CharacterTrait::Dwarf);
            },
            Class::Halfling => {
                traits.insert(CharacterTrait::Halfling);
            }
        }
        traits
    }
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub(crate) hit_points: u32,
    pub(crate) max_hit_points: u32,
    pub level: u32,
    pub weapon: Option<Equipment>,
    pub shield: Option<Equipment>,
    pub armor: Option<Equipment>,
    pub bow: Option<Equipment>,
    pub traits: HashSet<CharacterTrait>
}

impl Character {
    pub fn new(class: Class) -> Character {
        let max_hit_points = Self::calculate_hit_points(&class, 1);
        let traits = class.get_traits();
        Character {
            name: format!("Aventurero {}", class),
            class: class.clone(),
            hit_points: max_hit_points,
            max_hit_points,
            level: 1,
            weapon: None,
            shield: None,
            armor: None,
            bow: None,
            traits
        }
    }

    pub fn set_name(&mut self, name: String, existing_names: &HashSet<String>) {
        // Verificar que el nombre no esté vacío
        if name.trim().is_empty() {
            panic!("El nombre no puede estar vacío");
        }
        // Verificar que el nombre no esté duplicado
        if existing_names.contains(&name) {
            panic!("Ya existe un personaje con el nombre: {}", name);
        }
        self.name = name;
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
            EquipmentType::Bow => self.bow.replace(equipment),
            _ => None,
        }
    }

    pub fn unequip(&mut self, equipment_type: EquipmentType) -> Option<Equipment> {
        match equipment_type {
            EquipmentType::Weapon(_) => self.weapon.take(),
            EquipmentType::Shield => self.shield.take(),
            EquipmentType::Armor(_) => self.armor.take(),
            EquipmentType::Bow => self.bow.take(),
            _ => None,
        }
    }

    pub fn get_equipment_attack_bonus(&self) -> Option<i32> {
        if self.weapon.is_none() {
            return None;
        }
        let mut bonus = 0;
        
        // Bonus por arma
        if let Some(weapon) = &self.weapon {
            if weapon.equipment_type == EquipmentType::Weapon(WeaponType::Light) {
                bonus += -1;
            } else if weapon.equipment_type == EquipmentType::Weapon(WeaponType::Medium) {
                bonus += 0;
            } else if weapon.equipment_type == EquipmentType::Weapon(WeaponType::Heavy) {
                bonus += 1;
            }
        }

        Some(bonus)
    }

    pub fn get_equipment_defense_bonus(&self) -> i32 {
        let mut bonus = 0;

        // Bonus por armadura
        if let Some(armor) = &self.armor {
            if armor.equipment_type == EquipmentType::Armor(ArmorType::Light) {
                bonus += 1;
            } else if armor.equipment_type == EquipmentType::Armor(ArmorType::Heavy) {
                bonus += 2;
            }
        }

        // Bonus por escudo
        if let Some(shield) = &self.shield {
            bonus += 1;
        }

        bonus
    }

    pub fn get_class_defense_bonus(&self, enemy_tags: &[NPCTag]) -> i32 {
        let mut bonus = 0;

        // Bonus de clase y raza
        if matches!(&self.class, Class::Halfling | Class::Dwarf) && 
           enemy_tags.iter().any(|tag| matches!(tag, NPCTag::Troll | NPCTag::Ogre | NPCTag::Giant)) {
            bonus += 1;
        }

        bonus
    }

    pub fn get_class_attack_bonus(&self, enemies_outnumbered: bool, enemy_tags: &[NPCTag]) -> i32 {
        let is_two_handed = if let Some(weapon) = &self.weapon {
            matches!(weapon.equipment_type, EquipmentType::Weapon(WeaponType::Heavy))
        } else {
            false
        };

        let is_using_bow = if let Some(weapon) = &self.weapon {
            matches!(weapon.equipment_type, EquipmentType::Bow)
        } else {
            false
        };

        let mut bonus = match &self.class {
            Class::Fighter => self.level as i32,
            Class::Cleric => if enemy_tags.iter().any(|tag| tag == &NPCTag::Undead) {
                self.level as i32
            } else {
                (self.level as f32 / 2.0).floor() as i32
            },
            Class::Rogue => if enemies_outnumbered { self.level as i32 } else { 0 },
            Class::Wizard => 0,
            Class::Barbarian => self.level as i32,
            Class::Elf => if is_two_handed { 0 } else { self.level as i32 },
            Class::Dwarf => if is_using_bow { 0 } else { self.level as i32 },
            Class::Halfling => 0,
        };

        // Bonus adicional para Elfos contra orcos
        if matches!(&self.class, Class::Elf) && enemy_tags.iter().any(|tag| tag == &NPCTag::Orc) {
            bonus += 1;
        }
        if matches!(&self.class, Class::Dwarf) && enemy_tags.iter().any(|tag| tag == &NPCTag::Goblin) {
            bonus += 1;
        }

        bonus
    }

    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let actual_damage = damage.min(self.hit_points);
        self.hit_points = self.hit_points.saturating_sub(damage);
        actual_damage
    }

    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn has_trait(&self, trait_type: &CharacterTrait) -> bool {
        self.traits.contains(trait_type)
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
            _ => -2,
        }
    }
}

pub fn parse_new_character(input: String, existing_names: &HashSet<String>) -> Result<Character, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Formato incorrecto. Uso: nuevo <nombre> <clase>".to_string());
    }

    let name = parts[0].to_string();
    let class_str = parts[1].to_uppercase();

    let class = match class_str.as_str() {
        "GUERRERO" => Class::Fighter,
        "MAGO" => Class::Wizard,
        "PICARO" => Class::Rogue,
        _ => return Err("Clase no válida. Clases disponibles: GUERRERO, MAGO, PICARO".to_string()),
    };

    let mut character = Character::new(class);
    character.set_name(name, existing_names);
    Ok(character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Ya existe un personaje con el nombre: Gandalf")]
    fn test_duplicate_character_name() {
        let mut character = Character::new(Class::Wizard);
        let mut existing_names = HashSet::new();
        existing_names.insert("Gandalf".to_string());
        character.set_name("Gandalf".to_string(), &existing_names);
    }

    #[test]
    #[should_panic(expected = "El nombre no puede estar vacío")]
    fn test_empty_character_name() {
        let mut character = Character::new(Class::Wizard);
        let existing_names = HashSet::new();
        character.set_name("".to_string(), &existing_names);
    }
}