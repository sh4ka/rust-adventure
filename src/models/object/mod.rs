use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Object<'a> {
    description: String,
    tag: String,
    location: &'a Location,  // Referencia a una Location
}

impl Object<'_> {
    pub fn new(name: &str, location: &'_ Location) -> Self {
        Object {
            description: name.to_string(),
            tag: name.to_string(),
            location,
        }
    }
}

lazy_static! {
    pub static ref OBJECTS: Vec<Object> = vec![
        Object::new("una venda limpia.", "venda", "cueva"),
        Object::new("una cuerda en buen estado.", "cuerda", "campo"),
    ];
}
