use chrono::NaiveDate;
use clap::Args;

use crate::{
    commands::{next_revision, NextRevision, NextRoadTax},
    store::Store,
    Call, DTO,
};

use super::{NextRevisionDTO, NextRoadTaxDTO};

#[derive(Debug, Args)]
pub struct Checklist {
    #[arg(long)]
    plate: String,
}

#[derive(Debug)]
pub struct ChecklistDTO {
    plate: String,
    next_revision: NextRevisionDTO,
    next_road_tax: NextRoadTaxDTO,
}

impl DTO for ChecklistDTO {
    fn to_string(&self) -> String {
        format!(
            "Plate: {}\nNext Revision: {}\nNext Road Tax: {}",
            self.plate,
            self.next_revision.to_string(),
            self.next_road_tax.to_string()
        )
    }
}

impl Call for Checklist {
    type Output = ChecklistDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(output) => println!("{}", output.to_string()),
            Err(err) => eprintln!("{}", err),
        }
    }
    fn call(&self) -> Result<Self::Output, String> {
        let car_index = Store::find_by_plate(self.plate.as_str());

        match car_index {
            Some(index) => {
                let cars = Store::load();
                let car = &cars[index];

                let next_revision = NextRevision {
                    plate: car.plate.clone(),
                }
                .call()
                .unwrap();
                let next_road_tax = NextRoadTax {
                    plate: car.plate.clone(),
                }
                .call()
                .unwrap();

                Ok(ChecklistDTO {
                    plate: self.plate.clone(),
                    next_revision,
                    next_road_tax,
                })
            }
            None => Err("Plate is invalid".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{teardown, Add};

    fn setup() {
        Add {
            plate: "123".to_string(),
            brand: "Toyota".to_string(),
            owner: "Mateo".to_string(),
            last_revision: NaiveDate::from_ymd(2020, 1, 1),
            last_road_tax: NaiveDate::from_ymd(2020, 1, 1),
        }
        .call();
    }

    #[test]
    fn test_success() {
        setup();
        let cmd = Checklist {
            plate: "123".to_string(),
        };

        let res = cmd.call();
        assert!(res.is_ok());

        teardown();
    }

    #[test]
    fn test_failure() {
        let cmd = Checklist {
            plate: "1234".to_string(),
        };

        let res = cmd.call();
        assert_eq!(res.unwrap_err(), "Plate is invalid");
    }
}
