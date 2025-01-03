use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub owner: String,
    pub plate: String,
    pub brand: Option<String>,
}

pub struct Store {}

impl Store {
    pub fn load() -> Vec<Car> {
        let serialized_cars = include_str!("cars.json");
        serde_json::from_str(serialized_cars).expect("JSON wrong formatted")
    }

    pub fn save(cars: &[Car]) {
        dbg!(cars);
        let json_data = serde_json::to_string(&cars).expect("Unable to serialize cars");
        fs::write("cars.json", json_data).expect("Unable to write to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let cars = Store::load();
        assert_eq!(cars.len(), 0);
    }
}
