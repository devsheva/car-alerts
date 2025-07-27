use std::{fs, io};

pub static mut FILE_PATH: &str = "src/cars.json";

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub mod utils {

    use super::FILE_PATH;

    pub fn setup() {
        unsafe {
            FILE_PATH = "tests/cars.json";
        };
    }

    pub fn teardown() {
        std::fs::write(unsafe { FILE_PATH }, "[]\n").expect("Unable to reset file");
    }
}
