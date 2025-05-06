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
    Equip,
    Unequip,
    Unknown,
    Talk,
}

#[derive(Debug, Clone)]
pub struct GameCommand {
    command: Command,
    args: Vec<String>,
}

impl GameCommand {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            args: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
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
                let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
                GameCommand::new(Command::Go).with_args(args)
            } else {
                GameCommand::new(Command::Go)
            }
        },
        "coger" => {
            if words.len() > 1 {
                let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
                GameCommand::new(Command::Take).with_args(args)
            } else {
                GameCommand::new(Command::Take)
            }
        },
        "soltar" => {
            if words.len() > 1 {
                let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
                GameCommand::new(Command::Drop).with_args(args)
            } else {
                GameCommand::new(Command::Drop)
            }
        },
        "inventario" => GameCommand::new(Command::Inventory),
        "buscar" => GameCommand::new(Command::Search),
        "ayuda" => GameCommand::new(Command::Help),
        "salir" => GameCommand::new(Command::Exit),
        "estado" => GameCommand::new(Command::Status),
        "equipar" | "equip" => {
            let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
            GameCommand::new(Command::Equip).with_args(args)
        },
        "desequipar" | "unequip" => {
            let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
            GameCommand::new(Command::Unequip).with_args(args)
        },
        "atacar" => {
            if words.len() > 1 {
                let args: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
                GameCommand::new(Command::Attack).with_args(args)
            } else {
                GameCommand::new(Command::Attack)
            }
        },
        "1" => GameCommand::new(Command::Combat).with_args(vec!["continuar".to_string()]),
        "2" => GameCommand::new(Command::Combat).with_args(vec!["huir".to_string()]),
        "3" => GameCommand::new(Command::Combat).with_args(vec!["usar".to_string()]),
        "4" => GameCommand::new(Command::Combat).with_args(vec!["estado".to_string()]),
        _ => GameCommand::new(Command::Unknown),
    }
}

pub fn execute_command(player: &mut Player, command: GameCommand) -> String {
    match command.command {
        Command::Look => {
            player.execute_look()
        },
        Command::Go => {
            if let Some(direction) = command.args.first() {
                player.execute_go(Some(direction))
            } else {
                "¿A dónde quieres ir?".to_string()
            }
        },
        Command::Take => {
            if let Some(item) = command.args.first() {
                player.execute_take(item);
                "".to_string()
            } else {
                "¿Qué quieres tomar?".to_string()
            }
        },
        Command::Drop => {
            if let Some(item) = command.args.first() {
                player.execute_drop(item);
                "".to_string()
            } else {
                "¿Qué quieres soltar?".to_string()
            }
        },
        Command::Inventory => {
            player.execute_inventory();
            "".to_string()
        },
        Command::Equip => {
            let args: Vec<&str> = command.args.iter().map(|s| s.as_str()).collect();
            player.execute_equip(&args);
            "".to_string()
        },
        Command::Unequip => {
            let args: Vec<&str> = command.args.iter().map(|s| s.as_str()).collect();
            player.execute_unequip(&args);
            "".to_string()
        },
        Command::Talk => {
            if let Some(npc) = command.args.first() {
                player.execute_talk_to_npc(npc);
                "".to_string()
            } else {
                "¿Con quién quieres hablar?".to_string()
            }
        },
        Command::Attack => {
            if let Some(target) = command.args.first() {
                player.execute_attack(target)
            } else {
                "¿A quién quieres atacar?".to_string()
            }
        },
        Command::Exit => {
            println!("¡Hasta pronto!");
            std::process::exit(0);
            "".to_string()
        },
        Command::Status => {
            player.execute_status();
            "".to_string()
        },
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
            help.push_str("  equipar [personaje] <objeto> - Equipar un objeto (opcionalmente a un personaje específico)\n");
            help.push_str("  desequipar <objeto> - Desequipar un objeto\n");
            help.push_str("  ayuda - Mostrar esta ayuda\n");
            help.push_str("  salir - Salir del juego\n");
            help.push_str("\nDurante el combate:\n");
            help.push_str("  1 - Continuar el combate\n");
            help.push_str("  2 - Huir\n");
            help.push_str("  3 - Usar un objeto\n");
            help.push_str("  4 - Ver estado detallado");
            help
        },
        Command::Combat => {
            if let Some(action) = command.args.first() {
                match action.as_str() {
                    "continuar" => player.execute_attack("continuar"),
                    "huir" => {
                        "¡Huyes del combate!".to_string()
                    },
                    "usar" => {
                        "Función no implementada aún.".to_string()
                    },
                    "estado" => {
                        player.execute_status();
                        "".to_string()
                    },
                    _ => "Acción no válida.".to_string(),
                }
            } else {
                "¿Qué acción quieres realizar?".to_string()
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
