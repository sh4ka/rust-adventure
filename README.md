# Adventure Rust

A text-based adventure RPG game written in Rust. Explore the world of Woodspring, battle enemies, collect items, and uncover hidden secrets.

## Features

- **Character Creation**: Form a party of 4 adventurers from 8 different classes
- **Turn-based Combat**: Strategic combat system with different attack and defense mechanics
- **Inventory Management**: Collect, equip, and use items throughout your adventure
- **Exploration**: Discover locations, hidden passages, and solve puzzles
- **NPCs**: Interact with friendly and hostile characters in the game world
- **Class-specific Bonuses**: Each class has unique advantages against certain enemies

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)

### Steps

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/adventure-rust.git
   cd adventure-rust
   ```

2. Build and run the game:
   ```
   cargo run
   ```

## How to Play

### Basic Commands

- `mirar` - Look around your current location
- `ir [lugar]` - Travel to a connected location
- `coger [objeto]` - Pick up an item
- `soltar [objeto]` - Drop an item
- `inventario` - Check your inventory
- `buscar` - Search for hidden items or passages
- `estado` - Check your party's status
- `atacar` - Attack enemies in your location
- `hablar [npc]` - Talk to an NPC
- `equipar [personaje] [tipo]` - Equip an item to a character
- `desequipar [personaje] [tipo]` - Unequip an item from a character
- `ayuda` - Show available commands
- `salir` - Exit the game

### Combat

During combat:
1. Use numbers 1-4 to select combat actions
2. Combat is turn-based with your party attacking first, then enemies
3. Each character's attack and defense are determined by their class, equipment, and dice rolls
4. Enemies have different difficulty levels and may have special resistances or weaknesses

## Game World

The game takes place in and around the village of Woodspring. Key locations include:

- **Pueblo (Village)**: The starting location with friendly NPCs
- **Campo (Field)**: Open area connecting to other locations
- **Bosque (Forest)**: Contains hostile goblins and wolves
- **Ruinas (Ruins)**: Ancient temple ruins with orcs and skeletons
- **Cueva (Cave)**: Leads to a hidden dungeon with more challenging enemies

## Character Classes

The game features 8 playable classes, each with unique attributes:

- **Guerrero (Fighter)**: High HP, good with all weapons
- **Clérigo (Cleric)**: Medium HP, effective against undead
- **Pícaro (Rogue)**: Lower HP, bonus when outnumbering enemies
- **Mago (Wizard)**: Lowest HP, can use magical items
- **Bárbaro (Barbarian)**: Highest HP, strong in combat
- **Elfo (Elf)**: Medium HP, bonus against orcs, good with bows
- **Enano (Dwarf)**: High HP, bonus against goblins and large creatures
- **Mediano (Halfling)**: Lower HP, nimble and lucky

## Equipment Types

- **Weapons**: Light, Medium, and Heavy variants with different bonuses
- **Armor**: Light and Heavy variants providing different defense bonuses
- **Shields**: Provide defense bonuses
- **Bows**: Allow for ranged attacks

## Development

This project is built with:
- Rust programming language
- `lazy_static` for static references
- `rand` for random number generation

## License

This project is licensed under the MIT License - see the LICENSE file for details.