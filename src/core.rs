use std::{fs, io};

#[cfg(not(test))]
pub const FILE_PATH: &str = "src/cars.json";
#[cfg(test)]
pub const FILE_PATH: &str = "tests/cars.json";

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub mod utils {
    use super::FILE_PATH;

    pub fn teardown() {
        std::fs::write(FILE_PATH, "[]\n").expect("Unable to reset file");
    }
}
