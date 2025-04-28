#[derive(Debug)]
pub enum Command {
    Look,
    Go,
    Take(String),
    Unknown,
    Quit,
    Help,
}

pub fn parse_command(input: &str) -> Command {
    let input = input.trim().to_lowercase();
    let mut words = input.split_whitespace();
    
    match words.next() {
        Some("mirar") => Command::Look,
        Some("ir") => Command::Go,
        Some("coger") => Command::Take(words.collect::<Vec<&str>>().join(" ")),
        Some("ayuda") => Command::Help,
        Some("salir") => Command::Quit,
        _ => Command::Unknown,
    }
}

pub fn execute_command(command: Command) -> bool {
    match command {
        Command::Look => {
            println!("Miras a tu alrededor, pero está muy oscuro.");
            true
        }
        Command::Go => {
            println!("Está muy oscuro como para ir a ningún lugar.");
            true
        }
        Command::Take(item) => {
            println!("Intentas tomar {}", item);
            true
        }
        Command::Quit => {
            println!("¡Hasta luego!");
            false
        }
        Command::Help => {
            println!("Comandos básicos disponibles:");
            println!("- mirar: sirve para explorar tu entorno.");
            println!("- ir: sirve para moverse.");
            println!("- coger: sirve para tratar de coger un objeto.");
            println!("- salir: abandona el juego.");
            println!("- ayuda: imprime este texto.");
            true
        }
        Command::Unknown => {
            println!("No entiendo ese comando.");
            true
        }
    }
}
