use std::{fs, io};

#[cfg(not(test))]
pub const FILE_PATH: &str = "src/cars.json";
#[cfg(test)]
pub const FILE_PATH: &str = "tests/cars.json";

pub trait Call {
    fn call(&self);
}

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
