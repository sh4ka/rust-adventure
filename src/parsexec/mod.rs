use crate::models::player::Player;

#[derive(Debug, Clone)]
pub enum Command {
    Go(Option<String>),
    Look,
    Take(String),
    Drop(String),
    Inventory,
    Search,
    Status,
    Attack(String),
    Talk,
    Equip(Vec<String>),
    Unequip(Vec<String>),
    Salir,
    Help,
}

pub fn parse_command(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();
    match words.first() {
        Some(&"1") | Some(&"2") | Some(&"3") | Some(&"4") => Command::Attack(words[0].to_string()),
        Some(&"ir") => Command::Go(words.get(1).map(|&s| s.to_string())),
        Some(&"mirar") => Command::Look,
        Some(&"coger") => Command::Take(words.get(1).unwrap_or(&"").to_string()),
        Some(&"soltar") => Command::Drop(words.get(1).unwrap_or(&"").to_string()),
        Some(&"inventario") => Command::Inventory,
        Some(&"buscar") => Command::Search,
        Some(&"estado") => Command::Status,
        Some(&"atacar") => Command::Attack(words.get(1).unwrap_or(&"").to_string()),
        Some(&"hablar") => Command::Talk,
        Some(&"equipar") => Command::Equip(words[1..].iter().map(|&s| s.to_string()).collect()),
        Some(&"desequipar") => Command::Unequip(words[1..].iter().map(|&s| s.to_string()).collect()),
        Some(&"salir") => Command::Salir,
        Some(&"ayuda") => Command::Help,
        _ => Command::Look,
    }
}

pub fn execute_command(player: &mut Player, command: Command) -> String {
    // Si estamos en combate y el comando es un número, tratarlo como una acción de combate
    if player.current_combat_enemies.is_some() {
        if let Command::Attack(target) = &command {
            if let Ok(choice) = target.parse::<u32>() {
                if choice >= 1 && choice <= 4 {
                    return player.execute_attack(&target);
                }
            }
            // Si estamos en combate, ignorar otros comandos excepto los de combate
            return "No puedes hacer eso durante el combate. Usa los números 1-4 para las acciones de combate.".to_string();
        }
    }

    match command {
        Command::Go(location) => player.execute_go(location.as_deref()),
        Command::Look => player.execute_look(),
        Command::Take(item) => {
            if player.execute_take(&item) {
                "".to_string()
            } else {
                "No puedes coger ese objeto.".to_string()
            }
        },
        Command::Drop(item) => {
            if player.execute_drop(&item) {
                "".to_string()
            } else {
                "No puedes soltar ese objeto.".to_string()
            }
        },
        Command::Inventory => {
            player.execute_inventory();
            "".to_string()
        },
        Command::Search => {
            if player.execute_search() {
                "".to_string()
            } else {
                "No encuentras nada especial.".to_string()
            }
        },
        Command::Status => player.execute_status(),
        Command::Attack(target) => player.execute_attack(&target),
        Command::Talk => {
            println!("¿Con quién quieres hablar?");
            "".to_string()
        },
        Command::Equip(args) => {
            let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            if player.execute_equip(&args) {
                "".to_string()
            } else {
                "No puedes equipar ese objeto.".to_string()
            }
        },
        Command::Unequip(args) => {
            let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            if player.execute_unequip(&args) {
                "".to_string()
            } else {
                "No puedes desequipar ese objeto.".to_string()
            }
        },
        Command::Salir => {
            println!("¡Hasta pronto!");
            std::process::exit(0);
            "".to_string()
        },
        Command::Help => {
            let mut help = String::from("Comandos disponibles:\n");
            help.push_str("  mirar - Observar la ubicación actual\n");
            help.push_str("  ir [lugar] - Ir a una ubicación\n");
            help.push_str("  coger [objeto] - Recoger un objeto\n");
            help.push_str("  soltar [objeto] - Soltar un objeto\n");
            help.push_str("  inventario - Ver tu inventario\n");
            help.push_str("  buscar - Buscar objetos ocultos\n");
            help.push_str("  estado - Ver el estado del grupo\n");
            help.push_str("  atacar - Atacar a un enemigo\n");
            help.push_str("  hablar [npc] - Hablar con un NPC\n");
            help.push_str("  equipar [personaje] [tipo] - Equipar un objeto\n");
            help.push_str("  desequipar [personaje] [tipo] - Desequipar un objeto\n");
            help.push_str("  salir - Salir del juego\n");
            help
        },
        _ => "Comando no válido.".to_string(),
    }
} 