pub mod add;
pub use add::*;

pub mod list;
pub use list::*;

pub mod reset;
pub use reset::*;

pub mod next_revision;
pub use next_revision::*;

pub mod mark_revision;
pub use mark_revision::*;

pub mod utils {
    use crate::FILE_PATH;

    pub fn teardown() {
        std::fs::write(FILE_PATH, "[]\n").expect("Unable to reset file");
    }
}

pub use utils::*;
