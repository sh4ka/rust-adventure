use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::models::object::{NPC, Attitude, GameObject};

use super::object::NPCTag;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub base: NPC,
    pub difficulty: u8,        // 1-5, donde 5 es el más difícil
    pub min_level: u8,        // Nivel mínimo recomendado para enfrentarlo
    pub max_level: u8,        // Nivel máximo recomendado para enfrentarlo
    pub loot_table: Vec<String>, // Tags de los items que puede soltar
    pub experience: u32,      // Experiencia que otorga al derrotarlo
}

impl Enemy {
    pub fn new(
        tag: &str,
        description: &str,
        location: &str,
        difficulty: u8,
        min_level: u8,
        max_level: u8,
        experience: u32,
    ) -> Self {
        let base = NPC::new(tag, description, location, true)
            .with_attitude(Attitude::Hostile)
            .with_level(min_level);
        
        Self {
            base,
            difficulty: difficulty.min(5).max(1),
            min_level: min_level.min(20).max(1),
            max_level: max_level.min(20).max(min_level),
            loot_table: Vec::new(),
            experience,
        }
    }

    pub fn with_loot(mut self, item_tags: Vec<&str>) -> Self {
        self.loot_table = item_tags.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn with_count(mut self, count: u8) -> Self {
        self.base.count = count;
        self
    }

    pub fn with_tags(mut self, tags: Vec<NPCTag>) -> Self {
        self.base.tags = tags;
        self
    }
}

lazy_static! {
    pub static ref ENEMIES: HashMap<String, Enemy> = {
        let mut m = HashMap::new();
        
        // Enemigos del bosque
        m.insert("goblin".to_string(), 
            Enemy::new(
                "goblin",
                "un goblin pequeño y maloliente",
                "bosque",
                1,
                1,
                3,
                50
            )
            .with_loot(vec!["daga", "pocion_menor"])
            .with_count(4)
            .with_tags(vec![NPCTag::Goblin, NPCTag::Monster])
        );

        m.insert("lobo".to_string(),
            Enemy::new(
                "lobo",
                "un lobo salvaje y hambriento",
                "bosque",
                2,
                2,
                4,
                75
            )
            .with_loot(vec!["pocion_menor"])
            .with_count(3)
            .with_tags(vec![NPCTag::Beast, NPCTag::Monster])
        );

        // Enemigos de las ruinas
        m.insert("orco".to_string(),
            Enemy::new(
                "orco",
                "un orco musculoso y agresivo",
                "ruinas",
                3,
                3,
                5,
                100
            )
            .with_loot(vec!["hacha", "armadura", "pocion_menor"])
            .with_count(2)
            .with_tags(vec![NPCTag::Orc, NPCTag::Monster])
        );

        m.insert("esqueleto".to_string(),
            Enemy::new(
                "esqueleto",
                "un esqueleto animado con ojos brillantes",
                "ruinas",
                2,
                2,
                4,
                80
            )
            .with_loot(vec!["espada", "pocion_menor"])
            .with_count(3)
            .with_tags(vec![NPCTag::Undead, NPCTag::Monster])
        );

        // Enemigos de la mazmorra
        m.insert("troll".to_string(),
            Enemy::new(
                "troll",
                "un troll enorme y regenerativo",
                "mazmorra",
                4,
                4,
                6,
                150
            )
            .with_loot(vec!["hacha", "armadura_pesada", "pocion_mayor"])
            .with_count(1)
            .with_tags(vec![NPCTag::Troll, NPCTag::Monster])
        );

        m.insert("mimico".to_string(),
            Enemy::new(
                "mimico",
                "un cofre que resulta ser una criatura mimética",
                "mazmorra",
                3,
                3,
                5,
                120
            )
            .with_loot(vec!["pocion_mayor", "llave"])
            .with_count(1)
            .with_tags(vec![NPCTag::Monster])
        );

        // Jefe final
        m.insert("hechicero_oscuro".to_string(),
            Enemy::new(
                "hechicero_oscuro",
                "un poderoso hechicero vestido con ropajes oscuros",
                "torre",
                5,
                5,
                7,
                200
            )
            .with_loot(vec!["varita", "tunica", "pocion_mayor", "llave_maestra"])
            .with_count(1)
            .with_tags(vec![NPCTag::Human, NPCTag::Monster])
        );

        m
    };
}

pub fn get_enemy(tag: &str) -> Option<&'static Enemy> {
    ENEMIES.get(tag)
}

pub fn get_enemies_by_difficulty(difficulty: u8) -> Vec<&'static Enemy> {
    ENEMIES.values()
        .filter(|enemy| enemy.difficulty == difficulty)
        .collect()
}

pub fn get_enemies_by_level_range(min_level: u8, max_level: u8) -> Vec<&'static Enemy> {
    ENEMIES.values()
        .filter(|enemy| enemy.min_level >= min_level && enemy.max_level <= max_level)
        .collect()
}

pub fn get_enemies_by_location(location: &str) -> Vec<&'static Enemy> {
    ENEMIES.values()
        .filter(|enemy| enemy.base.location == location)
        .collect()
} 