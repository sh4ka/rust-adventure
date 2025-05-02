use lazy_static::lazy_static;
use crate::models::object::{find_location, find_item, find_npc, find_passage};
use crate::models::player::Player;

#[derive(Debug, Clone)]
pub enum Command {
    Look,
    Go,
    Take,
    Drop,
    Inventory,
    Search,
    Help,
    Exit,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct GameCommand {
    pub command: Command,
    pub target: Option<String>,  // Tag del objetivo (item, ubicación, etc.)
}

impl GameCommand {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            target: None,
        }
    }

    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }
}

pub fn parse_command(input: &str) -> GameCommand {
    let words: Vec<&str> = input.split_whitespace().collect();
    
    if words.is_empty() {
        return GameCommand::new(Command::Unknown);
    }

    match words[0].to_lowercase().as_str() {
        "mirar" => GameCommand::new(Command::Look),
        "ir" => {
            if words.len() > 1 {
                GameCommand::new(Command::Go).with_target(words[1].to_string())
            } else {
                GameCommand::new(Command::Go)
            }
        },
        "coger" => {
            if words.len() > 1 {
                GameCommand::new(Command::Take).with_target(words[1].to_string())
            } else {
                GameCommand::new(Command::Take)
            }
        },
        "soltar" => {
            if words.len() > 1 {
                GameCommand::new(Command::Drop).with_target(words[1].to_string())
            } else {
                GameCommand::new(Command::Drop)
            }
        },
        "inventario" => GameCommand::new(Command::Inventory),
        "buscar" => GameCommand::new(Command::Search),
        "ayuda" => GameCommand::new(Command::Help),
        "salir" => GameCommand::new(Command::Exit),
        _ => GameCommand::new(Command::Unknown),
    }
}

pub fn execute_command(command: GameCommand, player: &mut Player) {
    match command.command {
        Command::Look => {
            player.execute_look();
        },
        Command::Go => {
            if let Some(target) = command.target {
                if let Some(location) = find_location(&target) {
                    player.execute_go(Some(target));
                } else {
                    println!("No puedes ir allí.");
                }
            } else {
                println!("¿Ir a dónde?");
            }
        },
        Command::Take => {
            if let Some(target) = command.target {
                if let Some(item) = find_item(&target) {
                    player.execute_take(&item.base.tag);
                } else {
                    println!("No hay ningún objeto con ese nombre aquí.");
                }
            } else {
                println!("¿Coger qué?");
            }
        },
        Command::Drop => {
            if let Some(target) = command.target {
                if let Some(item) = find_item(&target) {
                    player.execute_drop(&item.base.tag);
                } else {
                    println!("No tienes ese objeto en tu inventario.");
                }
            } else {
                println!("¿Soltar qué?");
            }
        },
        Command::Inventory => {
            player.execute_inventory();
        },
        Command::Search => {
            player.execute_search();
        },
        Command::Help => {
            println!("Comandos disponibles:");
            println!("  mirar - Observar la ubicación actual");
            println!("  ir <lugar> - Ir a otro lugar");
            println!("  coger <objeto> - Coger un objeto");
            println!("  soltar <objeto> - Soltar un objeto");
            println!("  inventario - Ver tu inventario");
            println!("  buscar - Buscar objetos ocultos");
            println!("  ayuda - Mostrar esta ayuda");
            println!("  salir - Salir del juego");
        },
        Command::Exit => {
            println!("¡Hasta luego!");
            std::process::exit(0);
        },
        Command::Unknown => {
            println!("No entiendo ese comando. Escribe 'ayuda' para ver la lista de comandos disponibles.");
        },
    }
}

lazy_static! {
    pub static ref COMMANDS: Vec<GameCommand> = vec![
        GameCommand::new(Command::Look),
        GameCommand::new(Command::Go),
        GameCommand::new(Command::Take),
        GameCommand::new(Command::Drop),
        GameCommand::new(Command::Inventory),
        GameCommand::new(Command::Search),
        GameCommand::new(Command::Help),
        GameCommand::new(Command::Exit),
    ];
}
