use std::error::Error;
use lazy_static::lazy_static;
use crate::models::object::{find_object_by_tag, Object, LOCATIONS, OBJECTS};
use crate::models::player::Player;

#[derive(Debug, Clone)]
pub enum Command {
    Look,
    Go,
    Take,
    Unknown,
    Quit,
    Help,
    Inventory,
}

#[derive(Debug, Clone)]
pub struct GameCommand<'a> {
    command: Command,
    pub name: String,    
    pub description: String,
    pub available: bool,
    pub target: Option<&'a Object<'a>>
}

impl<'a> GameCommand<'a> {
    pub fn new(command: Command, name: &str, description: &str, available: bool) -> Self {
        GameCommand { 
            command,
            name: name.to_string(),
            description: description.to_string(),
            available,
            target: None
        }
    }

    pub fn with_target(mut self, target: &'a Object<'a>) -> Self {
        self.target = Some(target);
        self
    }
}

pub fn parse_command(input: &str) -> Option<GameCommand<'static>> {
    let input = input.trim().to_lowercase();
    let parts = input.split_whitespace().into_iter().collect::<Vec<&str>>();
    let command_name = parts[0];

    let mut game_command = find_command_by_name(command_name.to_string())?;
    
    // If there are additional words after the command, try to find a matching object
    if parts.len() > 1 {
        let target_name = parts[1..].join(" ");
        if let Some(target) = find_object_by_tag(&target_name) {
            game_command.target = Some(target);
        }
    }

    Some(game_command)
}

pub fn find_command_by_name<'a>(name: String) -> Option<GameCommand<'a>> {
    for command in COMMANDS.iter() {
        if command.name.to_lowercase() == name.to_lowercase() {
            return Some(command.clone());
        }
    }
    None
}

pub fn execute_command<'a>(player: &mut Player<'a>, game_command: &GameCommand<'a>) -> bool {
    let mut result = false;
    let command = &game_command.command;
    match command {
        Command::Look => {
            if let Some(target) = game_command.target {
                println!("Mirando {}", target.description);
            } else {
                player.execute_look();
            }
            result = true;
            result
        }
        Command::Go => {
            if let Some(target) = game_command.target {
                player.execute_go(Some(target));
                result = true;
            } else {
                println!("No se donde quieres ir. Escribe: Ir [lugar], por ejemplo 'Ir pueblo'");                
            }
            result
        }
        Command::Take => {
            if let Some(target) = game_command.target {
                result = player.execute_take(target);
            } else {
                println!("¿Qué quieres coger? Escribe: Coger [objeto]");
            }
            result
        }
        Command::Inventory => {
            player.execute_inventory();
            result = true;
            result
        }
        Command::Quit => {
            println!("¡Hasta luego!");
            std::process::exit(0);
        }
        Command::Help => {
            println!("Comandos básicos disponibles:");
            for command in COMMANDS.iter() {
                println!("- {}: {}", command.name, command.description);
            }
            result = true;
            result
        }
        Command::Unknown => {
            println!("No entiendo ese comando.");
            result = true;
            result
        }
    }
}

lazy_static! {
    pub static ref COMMANDS: Vec<GameCommand<'static>> = vec![
        GameCommand::new(Command::Look, "Mirar", "Mirar a tu alrededor.", true),
        GameCommand::new(Command::Go, "Ir", "Ir a un lugar.", true),
        GameCommand::new(Command::Take, "Coger", "Intentas coger algo.", true),
        GameCommand::new(Command::Inventory, "Inventario", "Muestra tu inventario.", true),
        GameCommand::new(Command::Help, "Ayuda", "Imprime este texto.", true),
        GameCommand::new(Command::Quit, "Salir", "Sale del juego.", true),
    ];
}
