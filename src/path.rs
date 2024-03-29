use crate::city;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    #[allow(dead_code)]
    pub from_city : *const city::City,
    pub to_city : *const city::City,
    pub pheromone : f32,
}

impl Path {
    pub fn new(from_city : &city::City, to_city : &city::City) -> Path {
        Path { from_city, to_city, pheromone: 0.50}
    }

    pub fn get_from_city (&self) -> &city::City {
        unsafe { &*self.from_city }
    }

    pub fn get_to_city (&self) -> &city::City {
        unsafe { &*self.to_city }
    }

    pub fn set_pheromone (&mut self, new_pheromone : f32) {
        self.pheromone = new_pheromone;
    }


    // Return the eucliden distance between citis.
    pub fn euclidean_distance (&self) -> f32 {
        let city1 = unsafe { (*self.from_city).clone() };
        let city2 = unsafe { (*self.to_city).clone() };
        let x_diff = city2.get_x_axis() - city1.get_x_axis();
        let y_diff = city2.get_y_axis() - city1.get_y_axis();
        let mut distance = x_diff.powf(2.0) + y_diff.powf(2.0);
        distance = distance.sqrt();
        return distance;
    }

}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}] : pheromone : {}", unsafe { (*self.from_city).name.clone() },  unsafe { (*self.to_city).name.clone() }, self.pheromone)
    }
}

impl PartialEq for Path {
    fn eq(&self, other : &Self) -> bool {
        self.from_city == other.from_city && self.to_city == other.to_city
    }
}
