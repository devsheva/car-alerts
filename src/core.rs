use std::{fmt::Debug, fs, io};

use clap::Args;

#[cfg(not(test))]
pub const FILE_PATH: &str = "src/cars.json";
#[cfg(test)]
pub const FILE_PATH: &str = "tests/cars.json";

pub trait DTO {
    fn to_string(&self) -> String;
}
pub trait Call: Args {
    type Output: DTO + Debug;

    fn call(&self) -> Result<Self::Output, String>;

    fn call_with_output(&self) {
        println!("Not implemented")
    }
}

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
