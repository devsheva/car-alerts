use crate::core::Call;
use crate::store::{Car, Store};
use crate::DTO;

use chrono::NaiveDate;
use clap::Args;

#[derive(Args)]
#[command(about = "Add a new car")]
pub struct Add {
    /// The owner of the car
    #[arg(short, long)]
    pub owner: String,

    /// The plate of the car
    #[arg(short, long)]
    pub plate: String,

    /// The brand of the car
    #[arg(short, long)]
    pub brand: String,

    /// Last revision date
    /// Format: yyyy-mm-dd
    #[arg(long)]
    pub last_revision: NaiveDate,

    /// Last road tax date
    /// Format: yyyy-mm-dd
    #[arg(long)]
    pub last_road_tax: NaiveDate,
}

#[derive(Debug)]
pub struct AddDTO {
    owner: String,
    plate: String,
    brand: String,
    last_revision: NaiveDate,
    last_road_tax: NaiveDate,
}

impl DTO for AddDTO {
    fn to_string(&self) -> String {
        format!(
            "Car added: owner: {}, plate: {}, brand: {}, last revision: {}, last road tax: {}",
            self.owner, self.plate, self.brand, self.last_revision, self.last_road_tax
        )
    }
}

impl Call for Add {
    type Output = AddDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(output) => println!("{}", output.to_string()),
            Err(error) => eprintln!("{}", error),
        }
    }

    fn call(&self) -> Result<AddDTO, String> {
        let mut cars = Store::load();

        let car = Car {
            owner: self.owner.clone(),
            plate: self.plate.clone(),
            brand: Some(self.brand.clone()),
            last_revision: self.last_revision,
            last_road_tax: self.last_road_tax,
        };

        cars.push(car);

        Store::save(&cars);

        Ok(AddDTO {
            owner: self.owner.clone(),
            plate: self.plate.clone(),
            brand: self.brand.clone(),
            last_revision: self.last_revision,
            last_road_tax: self.last_road_tax,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        let add = Add {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: "Toyota".to_string(),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        };

        add.call();

        assert_eq!(add.owner, "Mateo");
        assert_eq!(add.plate, "1234ABC");
        assert_eq!(add.brand, "Toyota");
    }

    #[test]
    fn test_add_duplicate_plate() {}
}
