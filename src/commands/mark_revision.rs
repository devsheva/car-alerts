use clap::Args;

use crate::{store::Store, Call, DTO};

#[derive(Args)]
#[command(about = "Mark a car revision")]
pub struct MarkRevision {
    /// Plate of the car
    #[arg(short, long)]
    pub plate: String,
}

#[derive(Debug, Default)]
pub struct MarkRevisionDTO {
    done: bool,
}

impl DTO for MarkRevisionDTO {
    fn to_string(&self) -> String {
        format!("MarkRevisionDTO {{ done: {} }}", self.done)
    }
}

impl Call for MarkRevision {
    type Output = MarkRevisionDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(dto) => println!("{}", dto.to_string()),
            Err(error) => eprintln!("{}", error),
        }
    }

    fn call(&self) -> Result<MarkRevisionDTO, String> {
        let mut cars = Store::load();

        let car = cars.iter_mut().find(|car| car.plate == self.plate);

        match car {
            Some(car) => {
                car.last_revision = chrono::Local::now().date_naive();
                Store::save(&cars);
                Ok(MarkRevisionDTO { done: true })
            }
            None => Err("Car not found".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::commands::{teardown, Add};

    use super::*;

    #[allow(unused_must_use)]
    fn setup() {
        let add = Add {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: "Toyota".to_string(),
            last_revision: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
            last_road_tax: NaiveDate::from_ymd_opt(2021, 10, 10).unwrap(),
        };

        add.call();
    }

    #[test]
    fn test_success() {
        setup();

        let cmd = MarkRevision {
            plate: "1234ABC".to_string(),
        };
        let result = cmd.call();

        assert_eq!(result.unwrap().done, true);

        let cars = Store::load();
        let car = cars.iter().find(|car| car.plate == "1234ABC").unwrap();
        assert_eq!(car.last_revision, chrono::Local::now().date_naive());

        teardown();
    }

    #[test]
    fn test_not_found() {
        let cmd = MarkRevision {
            plate: "not-found".to_string(),
        };
        let result = cmd.call();
        assert!(result.is_err());
    }
}
