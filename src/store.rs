use std::fs;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{read_file, FILE_PATH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub owner: String,
    pub plate: String,
    pub brand: Option<String>,
    pub last_revision: NaiveDate,
    pub last_road_tax: NaiveDate,
}

pub struct Store {}

impl Store {
    pub fn load() -> Vec<Car> {
        let serialized_cars = read_file(FILE_PATH).expect("Failed to read file");
        serde_json::from_str(&serialized_cars).expect("JSON wrong formatted")
    }

    pub fn save(cars: &[Car]) {
        let json_data = serde_json::to_string(&cars).expect("Unable to serialize cars");
        fs::write(FILE_PATH, json_data).expect("Unable to write to file");
    }

    pub fn find_by_plate(plate: &str) {
        todo!("implement")
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

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
            last_revision: Local::now().naive_local().date(),
            last_road_tax: Local::now().naive_local().date(),
        }];

        Store::save(&cars);
        assert_eq!(Store::load().len(), 1);

        teardown();
    }
}
