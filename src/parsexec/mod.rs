use crate::models::location::{execute_go, execute_look};
use crate::parsexec::Command::Look;
use crate::models::player::Player;

#[derive(Debug)]
pub enum Command {
    Look,
    Go(String),
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
        Some("ir") => Command::Go(words.collect::<Vec<&str>>().join(" ")),
        Some("coger") => Command::Take(words.collect::<Vec<&str>>().join(" ")),
        Some("ayuda") => Command::Help,
        Some("salir") => Command::Quit,
        _ => Command::Unknown,
    }
}

pub fn execute_command(player: &mut Player, command: Command) -> bool {
    match command {
        Command::Look => {
            execute_look(player);
            true
        }
        Command::Go(location) => {
            execute_go(location, player);
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
            println!("- ir: sirve para moverse. Ejemplo: Ir campo");
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
