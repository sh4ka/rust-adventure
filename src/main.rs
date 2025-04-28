mod parsexec;
use std::io::{self, Write};

const PROMPT: &str = "> ";

fn prompt() {
    print!("{}", PROMPT);
}

fn main() {
    println!("Bienvenido a la \"Aventura en la Cueva\".");
    println!("Está muy oscuro.");
    println!("(Escribe \"salir\" para salir, \"ayuda\" para lista de comandos básicos.)");

    loop {
        prompt();
        io::stdout().flush().unwrap();

        let mut input = String::new(); // creamos una nueva variable mutable de tipo string
        io::stdin().read_line(&mut input).unwrap(); // leemos una linea para obtener un comando

        let command = parsexec::parse_command(&input); // analizamos este comando
        if !parsexec::execute_command(command) { // procesamos el comando
            break;
        }
    }
}
