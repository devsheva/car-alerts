use chrono::NaiveDate;
use clap::Args;

use crate::{store::Store, Call, DTO};

#[derive(Args)]
#[command(about = "Get the next road tax date for a car")]
pub struct NextRoadTax {
    #[arg(short, long)]
    pub plate: String,
}

#[derive(Debug)]
pub struct NextRoadTaxDTO {
    plate: String,
    next_road_tax_date: NaiveDate,
}

impl DTO for NextRoadTaxDTO {
    fn to_string(&self) -> String {
        format!(
            "The next road tax date for car with plate {} is {}",
            self.plate, self.next_road_tax_date
        )
    }
}

impl Call for NextRoadTax {
    type Output = NextRoadTaxDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(dto) => println!("{}", dto.to_string()),
            Err(e) => println!("{}", e),
        }
    }

    fn call(&self) -> Result<Self::Output, String> {
        let cars = Store::load();

        let car = cars.iter().find(|car| car.plate == self.plate);

        match car {
            Some(car) => {
                let next_road_tax_date = car.last_road_tax + chrono::Months::new(12);

                Ok(NextRoadTaxDTO {
                    plate: car.plate.clone(),
                    next_road_tax_date,
                })
            }
            None => Err(format!("Car with plate {} not found", self.plate)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{teardown, Add};

    use super::*;

    #[test]
    fn test_not_found() {
        let cmd = NextRoadTax {
            plate: "not_found".to_string(),
        };

        let result = cmd.call();
        assert_eq!(result.unwrap_err(), "Car with plate not_found not found");
    }

    fn setup() {
        Add {
            owner: "John Doe".to_string(),
            brand: "Toyota".to_string(),
            last_revision: NaiveDate::from_ymd(2020, 1, 1),
            plate: "1234".to_string(),
            last_road_tax: NaiveDate::from_ymd(2020, 1, 1),
        }
        .call();
    }

    #[test]
    fn test_success() {
        setup();

        let cmd = NextRoadTax {
            plate: "1234".to_string(),
        };

        let result = cmd.call();
        assert_eq!(
            result.unwrap().next_road_tax_date,
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()
        );
        teardown();
    }
}
