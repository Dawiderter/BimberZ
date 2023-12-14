#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn print_error(&self) {
        eprintln!("ERROR: {}", self.message);
    }
}
