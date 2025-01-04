use clap::Args;

use crate::{store::Store, Call};

#[derive(Debug, Args)]
#[command(about = "List all cars")]
pub struct List {}

impl Call for List {
    fn call(&self) {
        let cars = Store::load();

        for car in cars {
            println!("{:?}", car);
        }
    }
}
