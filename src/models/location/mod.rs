use lazy_static::lazy_static;
use crate::Player;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Location {
    description: String,
    tag: String,
}

impl Location {
    pub fn new(description: &str, tag: &str) -> Location {
        Location {
            description: description.to_string(),
            tag: tag.to_string(),
        }
    }
}

pub fn execute_look(player: &Player) {
    if let Some(location) = LOCATIONS.get(player.current_location as usize) {
        println!("Estas en {}", location.description);
    } else {
        println!("Localización incorrecta.");
    }
}


pub fn execute_go(location: String, player: &mut Player) {
    for (i, loc) in LOCATIONS.iter().enumerate() {
        if location == loc.tag {
            let index_u32: u32 = i.try_into().unwrap();
            player.set_current_location(index_u32);
            execute_look(&player);
            break;
        }
    }
    println!("No se donde quieres ir.");
}

lazy_static! {
    pub static ref LOCATIONS: Vec<Location> = vec![
        Location::new("una pequeña cueva, fuera puedes ver campos de labranza.", "cueva"),
        Location::new("un campo abierto, recien labrado. En el horizonte puedes ver las humildes casas de Woodspring y cerca hay una pequeña cueva.", "campo"),
        Location::new("el pueblo de Woodspring, unas pocas casas de campesinos organizadas alrededor de una plaza. Puedes ver un pequeño comercio y una posada. Hay campos rodeando el pueblo.", "pueblo"),
    ];
}
