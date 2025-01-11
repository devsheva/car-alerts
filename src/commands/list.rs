use clap::Args;

use crate::{
    store::{Car, Store},
    Call, DTO,
};

#[derive(Debug, Args)]
#[command(about = "List all cars")]
pub struct List {}

#[derive(Debug)]
pub struct ListDTO {
    cars: Vec<Car>,
}

impl DTO for ListDTO {
    fn to_string(&self) -> String {
        format!("{:#?}", self.cars)
    }
}

impl Call for List {
    type Output = ListDTO;

    fn call_with_output(&self) {
        match self.call() {
            Ok(output) => println!("{}", output.to_string()),
            Err(error) => eprintln!("{}", error),
        }
    }

    fn call(&self) -> Result<ListDTO, String> {
        match Store::load() {
            cars if cars.is_empty() => Err("No cars found".to_string()),
            cars => Ok(ListDTO { cars }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let list = List {};
        let result = list.call();

        if let Err(error) = result {
            assert_eq!(error, "No cars found");
        }
    }
}
