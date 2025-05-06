use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum EquipmentType {
    Weapon,
    Shield,
    Armor
}

#[derive(Debug)]
pub struct Equipment {
    pub name: String,
    pub equipment_type: EquipmentType,
    pub bonus: i32,  // Bonus que proporciona el equipamiento
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
    pub weapon: Option<Equipment>,
    pub shield: Option<Equipment>,
    pub armor: Option<Equipment>,
}

impl Character {
    pub fn new(class: Class) -> Character {
        let max_hit_points = Self::calculate_hit_points(&class, 1);
        let mut character = Character {
            class: class.clone(),
            hit_points: max_hit_points,
            max_hit_points,
            level: 1,
            weapon: None,
            shield: None,
            armor: None,
        };

        // Asignar equipo inicial según la clase
        match class {
            Class::Fighter => {
                character.weapon = Some(Equipment {
                    name: "una espada de hierro".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: 0,
                });
                character.armor = Some(Equipment {
                    name: "una armadura de cuero".to_string(),
                    equipment_type: EquipmentType::Armor,
                    bonus: 1,
                });
            },
            Class::Cleric => {
                character.weapon = Some(Equipment {
                    name: "un hacha de batalla".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: 1,
                });
                character.shield = Some(Equipment {
                    name: "un escudo de madera".to_string(),
                    equipment_type: EquipmentType::Shield,
                    bonus: 1,
                });
            },
            Class::Rogue => {
                character.weapon = Some(Equipment {
                    name: "una daga ligera".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: -1,
                });
            },
            Class::Wizard => {
                character.weapon = Some(Equipment {
                    name: "una daga ligera".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: -1,
                });
            },
            Class::Barbarian => {
                character.weapon = Some(Equipment {
                    name: "un hacha de batalla".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: 1,
                });
            },
            Class::Elf => {
                character.weapon = Some(Equipment {
                    name: "una espada de hierro".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: 0,
                });
            },
            Class::Dwarf => {
                character.weapon = Some(Equipment {
                    name: "un hacha de batalla".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: 1,
                });
                character.armor = Some(Equipment {
                    name: "una armadura de cuero".to_string(),
                    equipment_type: EquipmentType::Armor,
                    bonus: 1,
                });
            },
            Class::Halfling => {
                character.weapon = Some(Equipment {
                    name: "una daga ligera".to_string(),
                    equipment_type: EquipmentType::Weapon,
                    bonus: -1,
                });
            },
        }

        character
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
        match equipment.equipment_type {
            EquipmentType::Weapon => {
                let old_weapon = self.weapon.take();
                self.weapon = Some(equipment);
                old_weapon
            },
            EquipmentType::Shield => {
                let old_shield = self.shield.take();
                self.shield = Some(equipment);
                old_shield
            },
            EquipmentType::Armor => {
                let old_armor = self.armor.take();
                self.armor = Some(equipment);
                old_armor
            },
        }
    }

    pub fn unequip(&mut self, equipment_type: EquipmentType) -> Option<Equipment> {
        match equipment_type {
            EquipmentType::Weapon => self.weapon.take(),
            EquipmentType::Shield => self.shield.take(),
            EquipmentType::Armor => self.armor.take(),
        }
    }

    pub fn get_equipment_bonus(&self) -> i32 {
        let mut bonus = 0;
        if let Some(weapon) = &self.weapon {
            bonus += weapon.bonus;
        }
        bonus
    }

    pub fn get_defense_bonus(&self) -> i32 {
        let mut bonus = 0;
        if let Some(shield) = &self.shield {
            bonus += shield.bonus;
        }
        if let Some(armor) = &self.armor {
            bonus += armor.bonus;
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