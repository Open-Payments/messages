#[derive(Debug)]
pub struct ValidationError {
    pub code: u32,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: u32, message: String) -> Self {
        ValidationError { code, message }
    }
}
