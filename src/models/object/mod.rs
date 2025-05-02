use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Location, // places you can go
    Item, // things you can pick up
    NPC, // beings you can interact with
    Passage, // passages between locations
}

#[derive(Debug, Clone)]
pub struct Object<'a> {
    pub tag: String,
    pub description: String,
    pub object_type: ObjectType,
    pub location: Option<&'a Object<'a>>,
    pub visible: bool,
    pub destination: Option<&'a Object<'a>>,
    pub picked_up: bool,
}

impl<'a> PartialEq for Object<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl<'a> Object<'a> {
    pub fn new(tag: &str, description: &str, object_type: ObjectType, location: Option<&'a Object<'a>>, visible: bool) -> Self {
        Object {
            tag: tag.to_string(),
            description: description.to_string(),
            object_type,
            location,
            visible,
            destination: None,
            picked_up: false,
        }
    }

    pub fn new_passage(tag: &str, description: &str, location: Option<&'a Object<'a>>, destination: &'a Object<'a>, visible: bool) -> Self {
        Object {
            tag: tag.to_string(),
            description: description.to_string(),
            location,
            object_type: ObjectType::Passage,
            visible,
            destination: Some(destination),
            picked_up: false,
        }
    }

    pub fn get_location(&self) -> Option<&'a Object<'a>> {
        self.location
    }

    pub fn get_destination(&self) -> Option<&'a Object<'a>> {
        self.destination
    }

    pub fn set_location(&mut self, new_location: Option<&'a Object<'a>>) {
        self.location = new_location;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_picked_up(&mut self, picked_up: bool) {
        self.picked_up = picked_up;
    }
}

pub fn find_object_by_tag(tag: &str) -> Option<&'static Object<'static>> {
    if let Some(location) = LOCATIONS.iter().find(|&obj| obj.tag == tag) {
        return Some(location);
    }

    if let Some(object) = OBJECTS.iter().find(|&obj| obj.tag == tag) {
        return Some(object);
    }

    None
}

lazy_static! {
    pub static ref LOCATIONS: Vec<Object<'static>> = vec![
        Object::new("cueva", "una pequeña cueva, con suelo de tierra y paredes de piedra, fuera puedes ver campos de labranza.", ObjectType::Location, None, true),
        Object::new("campo", "un campo abierto, recien labrado. En el horizonte puedes ver las humildes casas de Woodspring y cerca hay una pequeña cueva.", ObjectType::Location, None, true),
        Object::new("pueblo", "el pueblo de Woodspring, unas pocas casas de campesinos organizadas alrededor de una plaza. Puedes ver un pequeño comercio y una posada. Hay campos rodeando el pueblo.", ObjectType::Location, None, true),
        Object::new("bosque", "un bosque de robles a las afueras de Woodspring. Unas antiguas ruinas emergen de su umbral. Es de un tamaño magnífico y se extiende hasta el horizonte.", ObjectType::Location, None, true),
        Object::new("ruinas", "unas ruinas pertenecientes a un antiguo templo. La hiedra cubre gran parte de lo que antaño fueron majestuosas columnas de mármol.", ObjectType::Location, None, true),
        // New hidden rooms
        Object::new("camara-secreta", "una cámara secreta con paredes de piedra pulida. En el centro hay un pedestal antiguo con símbolos grabados.", ObjectType::Location, None, true),
        Object::new("laboratorio", "un laboratorio abandonado. Mesas de trabajo cubiertas de polvo y estantes con frascos de cristal se alinean en las paredes.", ObjectType::Location, None, true),
        Object::new("biblioteca", "una biblioteca oculta. Estanterías de madera antigua contienen tomos polvorientos y pergaminos enrollados.", ObjectType::Location, None, true),
        Object::new("tesoro", "una sala de tesoros. Cofres antiguos y estatuas de valor decoran esta cámara.", ObjectType::Location, None, true),
    ];
    
    pub static ref OBJECTS: Vec<Object<'static>> = {
        let location_cueva = &LOCATIONS[0];
        let location_campo = &LOCATIONS[1];
        let location_pueblo = &LOCATIONS[2];
        let location_camara = &LOCATIONS[5];
        let location_laboratorio = &LOCATIONS[6];
        let location_biblioteca = &LOCATIONS[7];
        let location_tesoro = &LOCATIONS[8];
        
        vec![
            Object::new("venda", "una venda limpia.", ObjectType::Item, Some(&location_cueva), false),
            Object::new("cuerda", "una cuerda en buen estado.", ObjectType::Item, Some(&location_campo), false),
            Object::new("moneda-plata-0", "una moneda de plata.", ObjectType::Item, Some(&location_campo), false),
            Object::new("antorcha", "una antorcha.", ObjectType::Item, Some(&location_cueva), true),
            Object::new("guardia", "una guardia de aspecto amable, armado con una lanza y armadura ligera de cuero.", ObjectType::NPC, Some(&location_pueblo), true),
            // Hidden passages
            Object::new_passage("grieta", "una grieta estrecha en la pared trasera de la cueva. Parece que se puede pasar por ella.", Some(&location_cueva), &location_camara, false),
            Object::new_passage("corredor", "un estrecho corredor, termina en una puerta de piedra con símbolos grabados.", Some(&location_cueva), &location_camara, false),
            Object::new_passage("puerta", "una puerta de piedra con símbolos grabados.", Some(&location_camara), &location_laboratorio, true),
            Object::new_passage("pasillo", "un pasillo oscuro que desciende.", Some(&location_laboratorio), &location_biblioteca, true),
            Object::new_passage("escalera", "una escalera de caracol que sube.", Some(&location_biblioteca), &location_tesoro, true),
        ]
    };
}
