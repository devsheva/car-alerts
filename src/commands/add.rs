use crate::core::Call;
use crate::store::{Car, Store};

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
}

impl Call for Add {
    fn call(&self) {
        let mut cars = Store::load();

        cars.push(Car {
            owner: self.owner.clone(),
            plate: self.plate.clone(),
            brand: Some(self.brand.clone()),
        });

        Store::save(&cars);
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
        };

        add.call();

        assert_eq!(add.owner, "Mateo");
        assert_eq!(add.plate, "1234ABC");
        assert_eq!(add.brand, "Toyota");
    }
}
