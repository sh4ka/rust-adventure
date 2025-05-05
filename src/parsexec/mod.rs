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
    Attack,
    Status,
    Combat,
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
        "estado" => GameCommand::new(Command::Status),
        "atacar" => {
            if words.len() > 1 {
                GameCommand::new(Command::Attack).with_target(words[1].to_string())
            } else {
                GameCommand::new(Command::Attack)
            }
        },
        "1" => GameCommand::new(Command::Combat).with_target("continuar".to_string()),
        "2" => GameCommand::new(Command::Combat).with_target("huir".to_string()),
        "3" => GameCommand::new(Command::Combat).with_target("usar".to_string()),
        "4" => GameCommand::new(Command::Combat).with_target("estado".to_string()),
        _ => GameCommand::new(Command::Unknown),
    }
}

pub fn execute_command(command: GameCommand, player: &mut Player) -> String {
    match command.command {
        Command::Look => player.execute_look(),
        Command::Go => player.execute_go(command.target.as_deref()),
        Command::Take => {
            if let Some(target) = command.target {
                if player.execute_take(&target) {
                    "".to_string()
                } else {
                    "No puedes coger eso.".to_string()
                }
            } else {
                "¿Qué quieres coger?".to_string()
            }
        },
        Command::Drop => {
            if let Some(target) = command.target {
                if player.execute_drop(&target) {
                    "".to_string()
                } else {
                    "No puedes soltar eso.".to_string()
                }
            } else {
                "¿Qué quieres soltar?".to_string()
            }
        },
        Command::Inventory => {
            player.execute_inventory();
            "".to_string()
        },
        Command::Status => player.execute_status(),
        Command::Search => {
            if player.execute_search() {
                "".to_string()
            } else {
                "No encuentras nada.".to_string()
            }
        },
        Command::Help => {
            let mut help = String::from("Comandos disponibles:\n");
            help.push_str("  mirar - Observar la ubicación actual\n");
            help.push_str("  ir <lugar> - Ir a otro lugar\n");
            help.push_str("  coger <objeto> - Coger un objeto\n");
            help.push_str("  soltar <objeto> - Soltar un objeto\n");
            help.push_str("  inventario - Ver tu inventario\n");
            help.push_str("  estado - Ver el estado del grupo\n");
            help.push_str("  buscar - Buscar objetos ocultos\n");
            help.push_str("  atacar <enemigo> - Atacar a un enemigo\n");
            help.push_str("  ayuda - Mostrar esta ayuda\n");
            help.push_str("  salir - Salir del juego\n");
            help.push_str("\nDurante el combate:\n");
            help.push_str("  1 - Continuar el combate\n");
            help.push_str("  2 - Huir\n");
            help.push_str("  3 - Usar un objeto\n");
            help.push_str("  4 - Ver estado detallado");
            help
        },
        Command::Exit => "¡Hasta luego!".to_string(),
        Command::Attack => {
            if let Some(target) = command.target {
                player.execute_attack(&target)
            } else {
                "¿A quién quieres atacar?".to_string()
            }
        },
        Command::Combat => {
            if let Some(action) = command.target {
                match action.as_str() {
                    "continuar" => player.execute_attack("continuar"),
                    "huir" => {
                        "¡Huyes del combate!".to_string()
                    },
                    "usar" => "Función no implementada aún.".to_string(),
                    "estado" => player.execute_status(),
                    _ => "Opción no válida.".to_string(),
                }
            } else {
                "Opción no válida.".to_string()
            }
        },
        Command::Unknown => "No entiendo ese comando.".to_string(),
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
