use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GameObject {
    pub tag: String,
    pub description: String,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub base: GameObject,
    pub connections: Vec<String>, // Tags de los pasajes que conectan con esta ubicación
}

#[derive(Debug, Clone)]
pub struct Item {
    pub base: GameObject,
    pub location: Option<String>, // Tag de la ubicación donde está el item
    pub is_dropped: bool,        // Si el item fue soltado por el jugador
}

#[derive(Debug, Clone)]
pub struct NPC {
    pub base: GameObject,
    pub location: String,        // Tag de la ubicación donde está el NPC
    pub dialogue: Vec<String>,   // Diálogos posibles del NPC
}

#[derive(Debug, Clone)]
pub struct Passage {
    pub base: GameObject,
    pub from: String,           // Tag de la ubicación de origen
    pub to: String,            // Tag de la ubicación de destino
    pub requires_key: bool,    // Si se necesita una llave para usar el pasaje
    pub key_tag: Option<String>, // Tag del item que funciona como llave
}

// Implementaciones para crear instancias
impl GameObject {
    pub fn new(tag: &str, description: &str, visible: bool) -> Self {
        Self {
            tag: tag.to_string(),
            description: description.to_string(),
            visible,
        }
    }
}

impl Location {
    pub fn new(tag: &str, description: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, passage_tag: &str) {
        self.connections.push(passage_tag.to_string());
    }
}

impl Item {
    pub fn new(tag: &str, description: &str, location: Option<&str>, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            location: location.map(|s| s.to_string()),
            is_dropped: false,
        }
    }
}

impl NPC {
    pub fn new(tag: &str, description: &str, location: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            location: location.to_string(),
            dialogue: Vec::new(),
        }
    }

    pub fn add_dialogue(&mut self, text: &str) {
        self.dialogue.push(text.to_string());
    }
}

impl Passage {
    pub fn new(tag: &str, description: &str, from: &str, to: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            from: from.to_string(),
            to: to.to_string(),
            requires_key: false,
            key_tag: None,
        }
    }

    pub fn with_key(mut self, key_tag: &str) -> Self {
        self.requires_key = true;
        self.key_tag = Some(key_tag.to_string());
        self
    }
}

// Estructuras globales para almacenar todas las entidades del juego
lazy_static! {
    pub static ref LOCATIONS: HashMap<String, Location> = {
        let mut m = HashMap::new();
        
        // Crear ubicaciones
        let mut cueva = Location::new("cueva", "una pequeña cueva, con suelo de tierra y paredes de piedra, fuera puedes ver campos de labranza.", true);
        let mut campo = Location::new("campo", "un campo abierto, recien labrado. En el horizonte puedes ver las humildes casas de Woodspring y cerca hay una pequeña cueva.", true);
        let mut pueblo = Location::new("pueblo", "el pueblo de Woodspring, unas pocas casas de campesinos organizadas alrededor de una plaza. Puedes ver un pequeño comercio y una posada. Hay campos rodeando el pueblo.", true);
        let mut bosque = Location::new("bosque", "un bosque de robles a las afueras de Woodspring. Unas antiguas ruinas emergen de su umbral. Es de un tamaño magnífico y se extiende hasta el horizonte.", true);
        let mut ruinas = Location::new("ruinas", "unas ruinas pertenecientes a un antiguo templo. La hiedra cubre gran parte de lo que antaño fueron majestuosas columnas de mármol.", true);
        
        // Ubicaciones ocultas
        let mut camara = Location::new("camara-secreta", "una cámara secreta con paredes de piedra pulida. En el centro hay un pedestal antiguo con símbolos grabados.", true);
        let mut laboratorio = Location::new("laboratorio", "un laboratorio abandonado. Mesas de trabajo cubiertas de polvo y estantes con frascos de cristal se alinean en las paredes.", true);
        let mut biblioteca = Location::new("biblioteca", "una biblioteca oculta. Estanterías de madera antigua contienen tomos polvorientos y pergaminos enrollados.", true);
        let mut tesoro = Location::new("tesoro", "una sala de tesoros. Cofres antiguos y estatuas de valor decoran esta cámara.", true);

        // Añadir ubicaciones al mapa
        m.insert("cueva".to_string(), cueva);
        m.insert("campo".to_string(), campo);
        m.insert("pueblo".to_string(), pueblo);
        m.insert("bosque".to_string(), bosque);
        m.insert("ruinas".to_string(), ruinas);
        m.insert("camara-secreta".to_string(), camara);
        m.insert("laboratorio".to_string(), laboratorio);
        m.insert("biblioteca".to_string(), biblioteca);
        m.insert("tesoro".to_string(), tesoro);

        m
    };

    pub static ref ITEMS: HashMap<String, Item> = {
        let mut m = HashMap::new();
        
        // Crear items
        m.insert("venda".to_string(), Item::new("venda", "una venda limpia", Some("cueva"), false));
        m.insert("cuerda".to_string(), Item::new("cuerda", "una cuerda en buen estado", Some("campo"), false));
        m.insert("moneda-plata-0".to_string(), Item::new("moneda-plata-0", "una moneda de plata", Some("campo"), false));
        m.insert("antorcha".to_string(), Item::new("antorcha", "una antorcha", Some("cueva"), true));

        m
    };

    pub static ref NPCS: HashMap<String, NPC> = {
        let mut m = HashMap::new();
        
        // Crear NPCs
        let mut guardia = NPC::new("guardia", "una guardia de aspecto amable, armado con una lanza y armadura ligera de cuero", "pueblo", true);
        guardia.add_dialogue("¡Bienvenido a Woodspring! ¿En qué puedo ayudarte?");
        guardia.add_dialogue("Ten cuidado en el bosque, dicen que hay criaturas extrañas.");
        
        m.insert("guardia".to_string(), guardia);

        m
    };

    pub static ref PASSAGES: HashMap<String, Passage> = {
        let mut m = HashMap::new();
        
        // Crear pasajes
        m.insert("grieta".to_string(), Passage::new("grieta", "una grieta estrecha en la pared trasera de la cueva. Parece que se puede pasar por ella", "cueva", "camara-secreta", false));
        m.insert("corredor".to_string(), Passage::new("corredor", "un estrecho corredor, termina en una puerta de piedra con símbolos grabados", "cueva", "camara-secreta", false));
        m.insert("puerta".to_string(), Passage::new("puerta", "una puerta de piedra con símbolos grabados", "camara-secreta", "laboratorio", true));
        m.insert("pasillo".to_string(), Passage::new("pasillo", "un pasillo oscuro que desciende", "laboratorio", "biblioteca", true));
        m.insert("escalera".to_string(), Passage::new("escalera", "una escalera de caracol que sube", "biblioteca", "tesoro", true));

        m
    };
}

// Funciones de utilidad para buscar entidades
pub fn find_location(tag: &str) -> Option<&'static Location> {
    LOCATIONS.get(tag)
}

pub fn find_item(tag: &str) -> Option<&'static Item> {
    ITEMS.get(tag)
}

pub fn find_npc(tag: &str) -> Option<&'static NPC> {
    NPCS.get(tag)
}

pub fn find_passage(tag: &str) -> Option<&'static Passage> {
    PASSAGES.get(tag)
}

// Función para obtener todos los items en una ubicación
pub fn get_items_in_location(location_tag: &str) -> Vec<&'static Item> {
    ITEMS.values()
        .filter(|item| item.location.as_ref().map_or(false, |loc| loc == location_tag))
        .collect()
}

// Función para obtener todos los NPCs en una ubicación
pub fn get_npcs_in_location(location_tag: &str) -> Vec<&'static NPC> {
    NPCS.values()
        .filter(|npc| npc.location == location_tag)
        .collect()
}

// Función para obtener todos los pasajes desde una ubicación
pub fn get_passages_from_location(location_tag: &str) -> Vec<&'static Passage> {
    PASSAGES.values()
        .filter(|passage| passage.from == location_tag)
        .collect()
}
