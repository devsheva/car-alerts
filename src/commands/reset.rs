use std::fs;

use clap::Args;

use crate::{Call, DTO, FILE_PATH};

#[derive(Args)]
pub struct Reset {}

#[derive(Debug)]
pub struct ResetDTO {
    done: bool,
}

impl DTO for ResetDTO {
    fn to_string(&self) -> String {
        match self.done {
            true => format!("Reset done"),
            false => format!("Reset failed"),
        }
    }
}
impl Call for Reset {
    type Output = ResetDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(output) => println!("{}", output.to_string()),
            Err(error) => eprintln!("{}", error),
        }
    }
    fn call(&self) -> Result<ResetDTO, String> {
        match fs::write(FILE_PATH, "[]") {
            Ok(_) => Ok(ResetDTO { done: true }),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::commands::Add;

    use super::*;

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
    fn test_reset() {
        setup();

        let content = fs::read_to_string(FILE_PATH).unwrap();
        assert_ne!(content, "[]");

        let reset = Reset {};
        let result = reset.call().unwrap();

        assert_eq!(result.done, true);
    }
}
