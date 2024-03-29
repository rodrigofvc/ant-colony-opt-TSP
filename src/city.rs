
use std::fmt;

#[derive(Debug, Clone)]
pub struct City {
    #[allow(dead_code)]
    pub id: u32,
    pub name: String,
    x_axis: f32,
    y_axis: f32,
}

impl City {

    pub fn new (id: u32, name: String, x_axis: f32, y_axis:f32) -> City {
        City { id, name, x_axis, y_axis}
    }

    pub fn get_x_axis (&self) -> f32 {
        self.x_axis
    }

    pub fn get_y_axis (&self) -> f32 {
        self.y_axis
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl PartialEq for City {
    fn eq(&self, other : &Self) -> bool {
        self.id == other.id && self.name == other.name && self.x_axis == other.x_axis && self.y_axis == other.y_axis
    }
}
