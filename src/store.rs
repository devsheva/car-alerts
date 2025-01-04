use std::fs;

use serde::{Deserialize, Serialize};

use crate::{read_file, FILE_PATH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub owner: String,
    pub plate: String,
    pub brand: Option<String>,
}

pub struct Store {}

impl Store {
    pub fn load() -> Vec<Car> {
        let serialized_cars = read_file(FILE_PATH).expect("Failed to read file");
        dbg!(&serialized_cars);
        serde_json::from_str(&serialized_cars).expect("JSON wrong formatted")
    }

    pub fn save(cars: &[Car]) {
        let json_data = serde_json::to_string(&cars).expect("Unable to serialize cars");
        fs::write(FILE_PATH, json_data).expect("Unable to write to file");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn teardown() {
        fs::write(FILE_PATH, "[]").expect("Unable to reset file");
    }

    #[test]
    fn test_load() {
        let cars = Store::load();
        assert_eq!(cars.len(), 0);
    }

    #[test]
    fn test_save() {
        let cars = vec![Car {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: Some("Toyota".to_string()),
        }];

        Store::save(&cars);
        assert_eq!(Store::load().len(), 1);

        teardown();
    }
}
