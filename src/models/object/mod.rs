use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::models::character::{Equipment, EquipmentType, WeaponType, ArmorType};

#[derive(Debug, Clone)]
pub struct GameObject {
    pub tag: String,
    pub description: String,
    pub long_description: String,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attitude {
    Hostile,
    Neutral,
    Friendly,
}

#[derive(Debug, Clone)]
pub struct RoomContent {
    pub items: Vec<Item>,      // Items en la sala
    pub hidden_items: Vec<Item>, // Items ocultos en la sala
    pub npcs: Vec<String>,     // Tags de los NPCs en la sala
    pub is_visited: bool,      // Si la sala ha sido visitada
    pub is_locked: bool,       // Si la sala está bloqueada
    pub required_key: Option<String>, // Tag del item necesario para desbloquear la sala
}

impl RoomContent {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            hidden_items: Vec::new(),
            npcs: Vec::new(),
            is_visited: false,
            is_locked: false,
            required_key: None,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn add_hidden_item(&mut self, item: Item) {
        self.hidden_items.push(item);
    }

    pub fn remove_item(&mut self, item_tag: &str) {
        self.items.retain(|item| item.base.tag != item_tag);
    }

    pub fn remove_hidden_item(&mut self, item_tag: &str) {
        self.hidden_items.retain(|item| item.base.tag != item_tag);
    }

    pub fn reveal_hidden_item(&mut self, item_tag: &str) -> Option<Item> {
        if let Some(index) = self.hidden_items.iter().position(|item| item.base.tag == item_tag) {
            let item = self.hidden_items.remove(index);
            self.items.push(item.clone());
            Some(item)
        } else {
            None
        }
    }

    pub fn find_hidden_item(&self, item_tag: &str) -> Option<&Item> {
        self.hidden_items.iter().find(|item| item.base.tag == item_tag)
    }

    pub fn add_npc(&mut self, npc_tag: &str) {
        self.npcs.push(npc_tag.to_string());
    }

    pub fn remove_npc(&mut self, npc_tag: &str) {
        self.npcs.retain(|tag| tag != npc_tag);
    }

    pub fn mark_as_visited(&mut self) {
        self.is_visited = true;
    }

    pub fn lock(&mut self, key_tag: Option<&str>) {
        self.is_locked = true;
        self.required_key = key_tag.map(|s| s.to_string());
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.required_key = None;
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub base: GameObject,
    pub connections: Vec<String>, // Tags de los pasajes que conectan con esta ubicación
    pub content: RoomContent,     // Contenido de la sala
}

#[derive(Debug, Clone)]
pub struct Item {
    pub base: GameObject,
    pub is_dropped: bool,        // Si el item fue soltado por el jugador
    pub is_equipment: bool,      // Si el item es equipable
    pub equipment_type: Option<EquipmentType>, // Tipo de equipamiento si es equipable
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NPCTag {
    // Razas
    Human,
    Elf,
    Dwarf,
    Orc,
    Goblin,
    Halfling,
    
    // Tipos
    Guard,
    Monster,
    Undead,
    Beast,
    Merchant,
    Bandit,
    Troll,
    Ogre,
    Giant,
    Vermin,    // Nuevo tipo de enemigo: alimañas/plagas
    
    // Alineamientos
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Debug, Clone)]
pub struct NPC {
    pub base: GameObject,
    pub location: String,        // Tag de la ubicación donde está el NPC
    pub dialogue: Vec<String>,   // Diálogos posibles del NPC
    pub attitude: Attitude,      // Actitud del NPC hacia el jugador
    pub level: u8,              // Nivel del NPC (1-20)
    pub count: u8,              // Cantidad de NPCs de este tipo
    pub tags: Vec<NPCTag>,      // Tags específicos del NPC
}

#[derive(Debug, Clone)]
pub struct Passage {
    pub base: GameObject,
    pub from: String,           // Tag de la ubicación de origen
    pub to: String,            // Tag de la ubicación de destino
    pub requires_item: bool,    // Si se necesita una llave para usar el pasaje
    pub item_tag: Option<String>,
    pub requires_key: bool,    // Si se necesita una llave para usar el pasaje
    pub key_tag: Option<String>, // Tag del item que funciona como llave
    pub has_riddle: bool,      // Si el pasaje tiene un acertijo
    pub riddle: Option<String>, // El acertijo
    pub riddle_answer: Option<String>, // La respuesta al acertijo
}

// Implementaciones para crear instancias
impl GameObject {
    pub fn new(tag: &str, description: &str, visible: bool) -> Self {
        Self {
            tag: tag.to_string(),
            description: description.to_string(),
            long_description: description.to_string(),
            visible,
        }
    }

    pub fn with_long_description(mut self, long_description: &str) -> Self {
        self.long_description = long_description.to_string();
        self
    }
}

impl Location {
    pub fn new(tag: &str, description: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            connections: Vec::new(),
            content: RoomContent::new(),
        }
    }

    pub fn add_connection(&mut self, passage_tag: &str) {
        self.connections.push(passage_tag.to_string());
    }

    pub fn with_long_description(mut self, long_description: &str) -> Self {
        self.base.long_description = long_description.to_string();
        self
    }
}

impl Item {
    pub fn new(tag: &str, description: &str) -> Self {
        Self {
            base: GameObject::new(tag, description, true),
            is_dropped: false,
            is_equipment: false,
            equipment_type: None,
        }
    }

    pub fn new_equipment(tag: &str, description: &str, is_equipment: bool, equipment_type: EquipmentType) -> Self {
        Self {
            base: GameObject::new(tag, description, true),
            is_dropped: false,
            is_equipment,
            equipment_type: Some(equipment_type),
        }
    }

    pub fn to_equipment(&self) -> Option<Equipment> {
        if self.is_equipment {
            Some(Equipment::new(self.base.description.clone(), self.equipment_type.clone()?))
        } else {
            None
        }
    }

    pub fn from_equipment(equipment: Equipment) -> Self {
        Self {
            base: GameObject::new(&equipment.name, &equipment.name, true),
            is_dropped: false,
            is_equipment: true,
            equipment_type: Some(equipment.equipment_type),
        }
    }
}

impl NPC {
    pub fn new(tag: &str, description: &str, location: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            location: location.to_string(),
            dialogue: Vec::new(),
            attitude: Attitude::Neutral, // Por defecto, los NPCs son neutrales
            level: 1,                    // Por defecto, los NPCs son nivel 1
            count: 1,                    // Por defecto, hay 1 NPC
            tags: Vec::new(),            // Inicialmente sin tags específicos
        }
    }

    pub fn with_attitude(mut self, attitude: Attitude) -> Self {
        self.attitude = attitude;
        self
    }

    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level.min(20).max(1); // Asegurar que el nivel esté entre 1 y 20
        self
    }

    pub fn with_count(mut self, count: u8) -> Self {
        self.count = count;
        self
    }

    pub fn add_dialogue(&mut self, text: &str) {
        self.dialogue.push(text.to_string());
    }

    pub fn add_tag(&mut self, tag: NPCTag) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn has_tag(&self, tag: &NPCTag) -> bool {
        self.tags.contains(tag)
    }
}

impl Passage {
    pub fn new(tag: &str, description: &str, from: &str, to: &str, visible: bool) -> Self {
        Self {
            base: GameObject::new(tag, description, visible),
            from: from.to_string(),
            to: to.to_string(),
            requires_item: false,
            item_tag: None,
            requires_key: false,
            key_tag: None,
            has_riddle: false,
            riddle: None,
            riddle_answer: None,
        }
    }

    pub fn with_item(mut self, item_tag: &str) -> Self {
        self.requires_item = true;
        self.item_tag = Some(item_tag.to_string());
        self
    }

    pub fn with_key(mut self, key_tag: &str) -> Self {
        self.requires_key = true;
        self.key_tag = Some(key_tag.to_string());
        self
    }

    pub fn with_riddle(mut self, riddle: &str, answer: &str) -> Self {
        self.has_riddle = true;
        self.riddle = Some(riddle.to_string());
        self.riddle_answer = Some(answer.to_string());
        self
    }
}

// Estructuras globales para almacenar todas las entidades del juego
lazy_static! {
    pub static ref LOCATIONS: HashMap<String, Location> = {
        let mut m = HashMap::new();
        
        // Crear ubicaciones
        let mut cueva = Location::new("cueva", "una pequeña cueva", true)
            .with_long_description("Una pequeña cueva con suelo de tierra y paredes de piedra. El aire es fresco y húmedo, y el eco de tus pasos resuena suavemente. Fuera puedes ver campos los de labranza de Woodspring.");
        let mut campo = Location::new("campo", "un campo abierto", true)
            .with_long_description("Un campo abierto, recién labrado. La tierra está fresca y húmeda, lista para la siembra. A poca distancia puedes ver las humildes casas de Woodspring y cerca hay una pequeña cueva que parece ser la entrada a algo más grande.");
        let mut pueblo = Location::new("pueblo", "el pueblo de Woodspring", true)
            .with_long_description("El pueblo de Woodspring, un asentamiento modesto pero acogedor. Unas pocas casas de campesinos se organizan alrededor de una plaza central. Puedes ver un pequeño comercio con su letrero desgastado y una posada con su chimenea humeante. Los campos rodean el pueblo, proporcionando sustento a sus habitantes.");
        let mut bosque = Location::new("bosque", "un bosque de robles", true)
            .with_long_description("Un bosque de robles a las afueras de Woodspring. Los árboles se elevan majestuosamente, sus ramas entrelazadas creando un dosel que filtra la luz del sol. Unas antiguas ruinas emergen de su umbral, sugiriendo una historia olvidada. El bosque se extiende hasta el horizonte, su tamaño es magnífico y su atmósfera, misteriosa.");
        let mut ruinas = Location::new("ruinas", "unas ruinas antiguas", true)
            .with_long_description("Unas ruinas pertenecientes a un antiguo templo dedicado a un dios olvidado. La hiedra cubre gran parte de lo que antaño fueron majestuosas columnas de mármol. Los símbolos grabados en las piedras están desgastados por el tiempo, pero aún se pueden distinguir algunos detalles. El aire aquí es más fresco y hay un silencio reverencial que sugiere que este lugar fue importante en el pasado.");
        
        // Ubicaciones ocultas
        let mut grieta = Location::new("grieta", "una grieta en la pared", false)
            .with_long_description("Una grieta estrecha en el fondo de la cueva. A través de ella se puede ver un corredor oscuro. El espacio es justo lo suficientemente grande para que una persona pueda pasar, pero requiere cierta agilidad. El aire que viene del otro lado es más frío y huele a humedad y antigüedad. El pasadizo está en completa oscuridad, no puedes entrar sin una fuente de luz.");
        let mut corredor = Location::new("corredor", "un corredor oscuro", true)
            .with_long_description("Un corredor estrecho y oscuro que termina en una puerta de piedra con símbolos grabados. Las paredes están húmedas y el suelo es irregular. La única iluminación proviene de la grieta por la que entraste, creando sombras que bailan en las paredes.");
        let mut puerta = Location::new("puerta", "una puerta de piedra", true)
            .with_long_description("Una pesada puerta de piedra con símbolos grabados. Los símbolos parecen contar una historia antigua, pero están parcialmente erosionados. La puerta parece estar sellada, pero hay un mecanismo que sugiere que puede ser abierta de alguna manera.");
        let mut camara = Location::new("camara", "una cámara abandonada", true)
            .with_long_description("Esta habtación parece haberse usado tiempo atrás como improvisado dormitorio y cocina. Hay una modesta mesa carcomida en una esquina. Una gruesa capa de polvo lo cubre todo.");
        let mut laboratorio = Location::new("laboratorio", "un laboratorio abandonado", true)
            .with_long_description("Un laboratorio abandonado que parece haber sido usado por alquimistas o magos. Mesas de trabajo cubiertas de polvo y estantes con frascos de cristal se alinean en las paredes. Algunos frascos aún contienen restos de líquidos de colores extraños, y hay notas y diagramas esparcidos por las mesas.");
        let mut biblioteca = Location::new("biblioteca", "una biblioteca antigua", true)
            .with_long_description("Una biblioteca antigua con estanterías de madera oscura que llegan hasta el techo. Los libros están cubiertos de polvo y algunos parecen estar escritos en idiomas olvidados. El aire huele a papel viejo y madera envejecida.");
        let mut tesoro = Location::new("tesoro", "una sala de tesoros", true)
            .with_long_description("Una sala de tesoros que parece haber pertenecido a alguien muy importante. Cofres antiguos y estatuas de valor decoran esta cámara. El oro y las gemas brillan a la luz de las antorchas, y el aire está cargado de la emoción de descubrir algo extraordinario.");

        // Añadir contenido a las ubicaciones
        cueva.content.add_item(Item::new("antorcha", "una antorcha"));
        campo.content.add_item(Item::new("cuerda", "una cuerda en buen estado"));
        campo.content.add_item(Item::new("moneda", "una moneda de plata"));
        biblioteca.content.add_item(Item::new("libro", "un libro de nigromancia, escrito en un idioma antiguo y bastante bien conservado. Anotado en un margen, está el nombre de un mago llamado 'Ainiriand'"));

        pueblo.content.add_npc("guardia");
        // Añadir grupos de NPCs a sus ubicaciones
        bosque.content.add_npc("goblins");
        ruinas.content.add_npc("orcos");
        laboratorio.content.add_npc("esqueletos");
        camara.content.add_npc("ratas");
        // Añadir conexiones
        pueblo.add_connection("campo");

        campo.add_connection("pueblo");
        campo.add_connection("cueva");

        cueva.add_connection("campo");
        cueva.add_connection("grieta"); // oculta
        cueva.add_connection("bosque");
        
        bosque.add_connection("cueva");
        bosque.add_connection("ruinas");
        
        ruinas.add_connection("bosque");

        // Localizaciones de la Cueva
        // grieta
        grieta.add_connection("cueva");
        grieta.add_connection("corredor");
        // corredor
        corredor.add_connection("grieta");
        corredor.add_connection("puerta");
        // puerta
        puerta.add_connection("corredor");
        puerta.add_connection("camara");
        // camara
        camara.add_connection("puerta");
        camara.add_connection("laboratorio");
        // laboratorio
        laboratorio.add_connection("camara");
        laboratorio.add_connection("biblioteca");
        // biblioteca
        biblioteca.add_connection("laboratorio");
        biblioteca.add_connection("tesoro");
        // tesoro
        tesoro.add_connection("biblioteca");

        // Añadir ubicaciones al mapa
        // localizaciones principales
        m.insert("cueva".to_string(), cueva);
        m.insert("campo".to_string(), campo);
        m.insert("pueblo".to_string(), pueblo);
        m.insert("bosque".to_string(), bosque);
        m.insert("ruinas".to_string(), ruinas);

        // Localizaciones de la Cueva
        m.insert("grieta".to_string(), grieta);
        m.insert("corredor".to_string(), corredor);
        m.insert("puerta".to_string(), puerta);
        m.insert("camara".to_string(), camara);
        m.insert("laboratorio".to_string(), laboratorio);
        m.insert("biblioteca".to_string(), biblioteca);
        m.insert("tesoro".to_string(), tesoro);

        m
    };

    pub static ref ITEMS: HashMap<String, Item> = {
        let mut m = HashMap::new();
        m.insert("espada".to_string(), Item::new_equipment("espada", "una espada de acero", true, EquipmentType::Weapon(WeaponType::Medium)));
        m.insert("daga".to_string(), Item::new_equipment("daga", "una daga ligera", true, EquipmentType::Weapon(WeaponType::Light)));
        m.insert("hacha".to_string(), Item::new_equipment("hacha", "un hacha de batalla", true, EquipmentType::Weapon(WeaponType::Heavy)));
        m.insert("escudo".to_string(), Item::new_equipment("escudo", "un escudo de madera", true, EquipmentType::Shield));
        m.insert("armadura".to_string(), Item::new_equipment("armadura", "una armadura de cuero", true, EquipmentType::Armor(ArmorType::Light)));
        m.insert("armadura_pesada".to_string(), Item::new_equipment("armadura_pesada", "una armadura de placas", true, EquipmentType::Armor(ArmorType::Heavy)));
        m.insert("antorcha".to_string(), Item::new("antorcha", "una antorcha encendida"));
        m.insert("llave".to_string(), Item::new("llave", "una llave de hierro"));
        m
    };

    pub static ref NPCS: HashMap<String, NPC> = {
        let mut m = HashMap::new();
        
        // Crear NPCs
        let mut guardia = NPC::new("guardia", "una guardia de aspecto amable, armada con una lanza y armadura ligera de cuero", "pueblo", true)
            .with_attitude(Attitude::Friendly)
            .with_level(3);
        guardia.add_dialogue("Bienvenido a Woodspring. ¿En qué puedo ayudarte?");
        guardia.add_dialogue("Ten cuidado en el bosque, dicen que hay criaturas extrañas.");
        guardia.add_tag(NPCTag::Human);
        guardia.add_tag(NPCTag::Guard);
        m.insert("guardia".to_string(), guardia);

        // Grupo de goblins en el bosque
        let mut goblins = NPC::new("goblins", "un grupo de goblins", "bosque", true)
            .with_attitude(Attitude::Hostile)
            .with_level(3)
            .with_count(4);
        goblins.add_tag(NPCTag::Goblin);
        goblins.add_tag(NPCTag::Monster);
        m.insert("goblins".to_string(), goblins);

        // Grupo de orcos en las ruinas
        let mut orcos = NPC::new("orcos", "un grupo de orcos", "ruinas", true)
            .with_attitude(Attitude::Hostile)
            .with_level(4)
            .with_count(7);
        orcos.add_tag(NPCTag::Orc);
        orcos.add_tag(NPCTag::Monster);
        m.insert("orcos".to_string(), orcos);

        let mut ratas = NPC::new("ratas", "un grupo de ratas hambrientas", "camara", true)
            .with_attitude(Attitude::Hostile)
            .with_level(1)
            .with_count(10);
        ratas.add_tag(NPCTag::Vermin);
        m.insert("ratas".to_string(), ratas);

        let mut esqueletos = NPC::new("esqueletos", "un grupo de esqueletos", "laboratorio", true)
            .with_attitude(Attitude::Hostile)
            .with_level(3)
            .with_count(6);
        esqueletos.add_tag(NPCTag::Undead);
        esqueletos.add_tag(NPCTag::Monster);
        m.insert("esqueletos".to_string(), esqueletos);
        
        m
    };

    pub static ref PASSAGES: HashMap<String, Passage> = {
        let mut m = HashMap::new();

        // Pasajes de la cueva
        m.insert("grieta".to_string(), Passage::new("grieta", "una grieta estrecha en la pared trasera de la cueva. Parece que se puede atravesar si encuentras una antorcha.", "cueva", "grieta", false));
        m.insert("corredor".to_string(), Passage::new("corredor", "un estrecho corredor, en completa oscuridad, es difícil ver lo que hay delante. Parece que se puede atravesar si encuentras una antorcha.", "grieta", "corredor", false).with_item("antorcha"));
        m.insert("puerta".to_string(), Passage::new("puerta", "una puerta de piedra con símbolos grabados", "puerta", "camara", true)
            .with_riddle(
                "Soy alto cuando soy joven y bajo cuando soy viejo. ¿Qué soy?",
                "vela"
            ));
        m.insert("laboratorio".to_string(), Passage::new("laboratorio", "un pasillo que conduce al laboratorio", "camara", "laboratorio", true));
        m.insert("biblioteca".to_string(), Passage::new("biblioteca", "un pasillo que conduce a la biblioteca", "laboratorio", "biblioteca", true));
        m.insert("tesoro".to_string(), Passage::new("tesoro", "una trampilla que conduce a una pequeña sala de tesoros", "biblioteca", "tesoro", true).with_key("llave"));

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
        .filter(|item| item.base.tag == location_tag)
        .collect()
}

// Función para obtener todos los items ocultos en una ubicación
pub fn get_hidden_items_in_location(location: &Location) -> Vec<&Item> {
    location.content.hidden_items.iter().collect()
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

pub fn find_item_in_location<'a>(location: &'a Location, item_tag: &str) -> Option<&'a Item> {
    location.content.items.iter().find(|item| item.base.tag == item_tag)
}
