use std::fs;

use clap::Args;

use crate::{Call, FILE_PATH};

#[derive(Args)]
pub struct Reset {}

impl Call for Reset {
    fn call(&self) {
        fs::write(FILE_PATH, "[]").expect("Unable to reset file");
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::Add;

    use super::*;

    fn setup() {
        let add = Add {
            owner: "Mateo".to_string(),
            plate: "1234ABC".to_string(),
            brand: "Toyota".to_string(),
        };

        add.call();
    }

    #[test]
    fn test_reset() {
        setup();

        let content = fs::read_to_string(FILE_PATH).unwrap();
        assert_ne!(content, "[]");

        let reset = Reset {};
        reset.call();

        let content = fs::read_to_string(FILE_PATH).unwrap();
        assert_eq!(content, "[]");
    }
}
