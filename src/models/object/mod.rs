use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Location, // places you can go
    Item, // things you can pick up
    NPC, // beings you can interact with
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object<'a> {
    pub description: String,
    pub(crate) tag: String,
    location: Option<&'a Object<'a>>,  // Referencia a una Location en la que este object se encuentra
    pub object_type: ObjectType,
    visible: bool,
}

impl<'a> Object<'a> {
    pub fn new(description: &str, tag: &str, location: Option<&'a Object<'a>>, object_type: ObjectType, visible: bool) -> Self {
        Object {
            description: description.to_string(),
            tag: tag.to_string(),
            location,
            object_type,
            visible,
        }
    }

    pub fn get_location(&self) -> Option<&'a Object<'a>> {
        self.location
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

pub fn find_objects_by_location(tag: &str) -> Vec<&'static Object<'static>> {
    if let Some(location) = LOCATIONS.iter().find(|&loc| loc.tag == tag) {
        return OBJECTS.iter()
            .filter(|&obj| obj.get_location() == Some(location) && obj.visible)
            .collect();
    }

    // Si no se encuentra la ubicación, devolver una lista vacía
    Vec::new()
}

lazy_static! {
    pub static ref LOCATIONS: Vec<Object<'static>> = vec![
        Object::new("una pequeña cueva, con suelo de tierra y paredes de piedra, fuera puedes ver campos de labranza.", "cueva", None, ObjectType::Location, true),
        Object::new("un campo abierto, recien labrado. En el horizonte puedes ver las humildes casas de Woodspring y cerca hay una pequeña cueva.", "campo", None, ObjectType::Location, true),
        Object::new("el pueblo de Woodspring, unas pocas casas de campesinos organizadas alrededor de una plaza. Puedes ver un pequeño comercio y una posada. Hay campos rodeando el pueblo.", "pueblo", None, ObjectType::Location, true),
        Object::new("un bosque de robles a las afueras de Woodspring. Unas antiguas ruinas emergen de su umbral. Es de un tamaño magnífico y se extiende hasta el horizonte.", "bosque", None, ObjectType::Location, true),
        Object::new("unas ruinas pertenecientes a un antiguo templo. La hiedra cubre gran parte de lo que antaño fueron majestuosas columnas de mármol.", "ruinas", None, ObjectType::Location, true),
    ];
    
    pub static ref OBJECTS: Vec<Object<'static>> = {
        let location_cueva = &LOCATIONS[0];
        let location_campo = &LOCATIONS[1];
        let location_pueblo = &LOCATIONS[2];
        
        vec![
            Object::new("una venda limpia.", "venda", Some(&location_cueva), ObjectType::Item, false),
            Object::new("una cuerda en buen estado.", "cuerda", Some(&location_campo), ObjectType::Item, false),
            Object::new("una moneda de plata.", "moneda-plata-0", Some(&location_campo), ObjectType::Item, false),
            Object::new("una antorcha.", "antorcha", Some(&location_cueva), ObjectType::Item, true),
            Object::new("una guardia de aspecto amable, armado con una lanza y armadura ligera de cuero.", "guardia", Some(&location_pueblo), ObjectType::NPC, true),
        ]
    };
}
