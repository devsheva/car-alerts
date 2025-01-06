use clap::Args;

use crate::{store::Store, Call, DTO};

#[derive(Args)]
pub struct NextRevision {
    /// The plate of the car
    #[arg(short, long)]
    pub plate: String,
}

#[derive(Debug)]
pub struct NextRevisionDTO {
    plate: String,
    brand: Option<String>,
    next_revision: chrono::NaiveDate,
}

impl DTO for NextRevisionDTO {
    fn to_string(&self) -> String {
        format!(
            "The next revision for car {} with brand: {}, is {} ",
            self.plate,
            self.brand.as_deref().unwrap(),
            self.next_revision
        )
    }
}

impl Call for NextRevision {
    type Output = NextRevisionDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(output) => println!("{}", output.to_string()),
            Err(error) => eprintln!("{}", error),
        }
    }

    fn call(&self) -> Result<NextRevisionDTO, String> {
        println!("Next revision");

        let cars = Store::load();

        match cars.iter().find(|car| car.plate == self.plate) {
            Some(car) => {
                let next_revision = car.last_revision + chrono::Months::new(24);

                Ok(NextRevisionDTO {
                    plate: car.plate.clone(),
                    brand: car.brand.clone(),
                    next_revision: next_revision.clone(),
                })
            }
            None => Err("Car not found".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{teardown, Add};

    use chrono::NaiveDate;

    use super::*;

    fn setup() {
        Add {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: "Toyota".to_string(),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        }
        .call();
    }

    #[test]
    fn test_next_revision() {
        setup();

        let cmd = NextRevision {
            plate: "1234ABC".to_string(),
        };

        let result = cmd.call().unwrap();
        assert_eq!(result.plate, "1234ABC");

        teardown();
    }

    #[test]
    fn test_not_found() {
        let cmd = NextRevision {
            plate: "1234ABC".to_string(),
        };

        let output = cmd.call();
        assert_eq!(output.unwrap_err(), "Car not found");
    }
}
